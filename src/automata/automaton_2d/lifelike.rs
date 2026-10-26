use std::time::Instant;

use macroquad::{color::{GREEN, WHITE}, input::{KeyCode, MouseButton, is_key_down, is_key_pressed, is_mouse_button_down, mouse_position, mouse_wheel}, shapes::{draw_rectangle, draw_rectangle_lines}, window::screen_width};

use crate::{automata::automaton_2d::base::{Automaton2D, Automaton2DInfo}, helpers::{delayer::Delayer, timer::Timer}};

#[derive(Copy, Clone)]
pub enum LifelikeConfig {
    None,
    AllRandom,
    RandomCenter(usize)
}

#[derive(Copy, Clone)]
pub enum LifelikeNeighborhoodType {
    Moore,
    VonNeumann
}

pub struct Lifelike {
    pub info: Automaton2DInfo<bool>,
    pub config: LifelikeConfig,
    pub neighborhood: LifelikeNeighborhoodType,
    pub rule: (u16, u16),
    pub timer: Timer,
    pub advance_delayer: Delayer,
    pub move_delayer: Delayer,

    is_editing: bool,
    editing_cell_state: bool, // true if the cell is being changed to alive, false if dead
    created: Instant,
}

impl Automaton2D<bool, LifelikeConfig, (u16, u16)> for Lifelike {
    fn new(width: usize, height: usize, config: LifelikeConfig, rule: (u16, u16)) -> Self {
        if width < 100 {
            panic!("Automaton should not have a width less than 100");
        }
        if height < 100 {
            panic!("Automaton should not have a height less than 100");
        }

        let info: Automaton2DInfo<bool> = Automaton2DInfo::new(width, height, (width / 2 + 1 - 50, width / 2 + 1 + 50));

        let mut s = Self {
            info,
            config,
            rule,
            neighborhood: LifelikeNeighborhoodType::Moore,
            timer: Timer::new(),
            advance_delayer: Delayer::new(None),
            move_delayer: Delayer::new(None),

            is_editing: false,
            editing_cell_state: false,
            created: Instant::now(),
        };

        s.apply_config(config);

        s
    }

    fn apply_config(&mut self, config: LifelikeConfig) {
        let center_x_index = self.info.data1[0].len() / 2 + 1;
        let center_y_index = self.info.data1.len() / 2 + 1;

        self.config = config;

        match config {
            LifelikeConfig::AllRandom => {
                for y in 0..self.info.data1.len() {
                    for x in 0..self.info.data1[0].len() {
                        self.info.data1[y][x] = rand::random_bool(0.5);
                    }
                }
            }
            LifelikeConfig::RandomCenter(size) => {
                for y in center_y_index-(size/2)..center_y_index+(size/2) {
                    for x in center_x_index-(size/2)..center_x_index+(size/2) {
                        self.info.data1[y][x] = rand::random_bool(0.5);
                    }
                }
            }
            _ => {}
        }

        self.created = Instant::now();
    }

    fn restart(&mut self) {
        let width = self.info.data1[0].len();
        let height = self.info.data1.len();
        let seen_range_x = (width / 2 + 1 - 50, width / 2 + 1 + 50);
        
        self.info = Automaton2DInfo::new(width, height, seen_range_x);
        self.apply_config(self.config);
    }

    fn pass_time(&mut self) {
        let to_read = match self.neighborhood {
            LifelikeNeighborhoodType::Moore => {
                vec![
                    (-1, -1),(0, -1),(1, -1),
                    (-1, 0),(1, 0),
                    (-1, 1),(0, 1),(1, 1)
                ]
            }
            LifelikeNeighborhoodType::VonNeumann => {
                vec![
                    (0, -1),
                    (-1, 0), (1, 0),
                    (0, 1)
                ]
            }
        };

        for y in 0..self.info.get_height() {
            for x in 0..self.info.get_width() {
                let mut count: u16 = 0;

                for (sx, sy) in to_read.iter() {
                    if self.info.read_from(x as i32 + sx, y as i32 + sy) {
                        count += 1;
                    }
                }

                if !self.info.read_from(x as i32, y as i32) { // current cell is dead
                    if self.rule.0 & (1 << count) > 0 {
                        self.info.write_data(x as i32, y as i32, true);
                    } else {
                        self.info.write_data(x as i32, y as i32, false);
                    }
                } else { // current cell is alive
                    if self.rule.1 & (1 << count) > 0 {
                        self.info.write_data(x as i32, y as i32, true);
                    } else {
                        self.info.write_data(x as i32, y as i32, false);
                    }
                }
            }
        }

        self.info.increment_time();
    }

    fn move_window(&mut self, direction: (i32, i32)) {
        self.info.move_window(direction);
    }

    fn control(&mut self) {
        let mut editable = self.created.elapsed().as_millis() > 200;

        let mw = mouse_wheel();
        if mw.1 < 0.0 {
            self.info.zoom_out();
        } 
        if mw.1 > 0.0 {
            self.info.zoom_in();
        }

        if self.advance_delayer.run_delayer(&[KeyCode::Space]) && self.timer.run_timer(&[KeyCode::Space]) {
            editable = false;
            self.pass_time();
        }

        let move_keys = [
            KeyCode::Up,
            KeyCode::Left, 
            KeyCode::Down,
            KeyCode::Right,
            KeyCode::W,
            KeyCode::A,
            KeyCode::S,
            KeyCode::D
        ];

        if self.move_delayer.run_delayer(&move_keys) && self.timer.run_timer(&move_keys) {
            editable = false;
            let mut movement = (0, 0);
            if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                movement.1 -= 1;
            }
            if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                movement.0 -= 1;
            }
            if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                movement.1 += 1;
            }
            if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                movement.0 += 1;
            }

            self.move_window(movement);
        }

        if is_key_pressed(KeyCode::R) {
            self.restart();
        }

        if editable {
            if !is_mouse_button_down(MouseButton::Left) {
                self.is_editing = false;
            } else {
                if !self.is_editing {
                    self.editing_cell_state = !self.info.mouse_get_cell();
                    self.is_editing = true;
                }

                self.info.mouse_set_cell(self.editing_cell_state);
            }
        } else {
            self.is_editing = false;
        }
    }

    fn draw(&self) {
        let cell_width = screen_width() / (self.info.seen_range_x.1 - self.info.seen_range_x.0) as f32;
    
        for y in self.info.seen_range_y.0..self.info.seen_range_y.1 {
            for x in self.info.seen_range_x.0..self.info.seen_range_x.1 {
                let x_position = ((x - self.info.seen_range_x.0) as f32) * cell_width;
                let y_position = ((y - self.info.seen_range_y.0) as f32) * cell_width;

                if self.info.read_from(x as i32, y as i32){
                    draw_rectangle(x_position, y_position, cell_width, cell_width, WHITE);
                }
            }
        }

        let mouse_pos = mouse_position();
        let cell_size = self.info.get_cell_size();
        let cell_pos = (
            (mouse_pos.0 / cell_size).floor() * cell_size,
            (mouse_pos.1 / cell_size).floor() * cell_size
        );
        draw_rectangle_lines(
            cell_pos.0, cell_pos.1, cell_size, cell_size, 
            1.0, GREEN
        );
    }
}

impl Lifelike {
    pub fn set_neighborhood(&mut self, neighborhood: LifelikeNeighborhoodType) {
        self.neighborhood = neighborhood;
    }
}