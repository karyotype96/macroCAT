use std::time::Instant;

use macroquad::{color::{BLUE, GREEN, RED, WHITE}, input::{KeyCode, MouseButton, is_key_down, is_key_pressed, is_mouse_button_down, mouse_position, mouse_wheel}, shapes::{draw_rectangle, draw_rectangle_lines}, window::screen_width};
use rand::distr::{Distribution, weighted::WeightedIndex};

use crate::{automata::automaton_2d::{base::{Automaton2D, Automaton2DInfo}, brians_brain::DummyRule}, helpers::{delayer::Delayer, timer::Timer}};

#[derive(Copy, Clone)]
pub enum RPSConfig {
    AllRandom,
    RandomCenter(usize)
}

#[derive(Copy, Clone)]
pub enum RPSNeighborhoodType {
    Moore,
    VonNeumann
}

#[derive(Copy, Clone, Default, PartialEq, PartialOrd)]
pub enum RPSState {
    #[default]
    None,
    Rock,
    Paper,
    Scissors
}

pub struct RPS {
    pub info: Automaton2DInfo<RPSState>,
    pub config: RPSConfig,
    pub neighborhood: RPSNeighborhoodType,
    pub timer: Timer,
    pub advance_delayer: Delayer,
    pub move_delayer: Delayer,

    editing_cell_state: RPSState,
    created: Instant,
}

impl Automaton2D<RPSState, RPSConfig, DummyRule> for RPS {
    fn new(width: usize, height: usize, config: RPSConfig, _rule: DummyRule) -> Self {
        if width < 100 {
            panic!("Automaton should not have a width less than 100");
        }
        if height < 100 {
            panic!("Automaton should not have a height less than 100");
        }

        let info: Automaton2DInfo<RPSState> = Automaton2DInfo::new(width, height, (width / 2 + 1 - 50, width / 2 + 1 + 50));

        let mut s = Self {
            info,
            config,
            neighborhood: RPSNeighborhoodType::Moore,
            timer: Timer::new(),
            advance_delayer: Delayer::new(None),
            move_delayer: Delayer::new(None),

            editing_cell_state: RPSState::None,
            created: Instant::now(),
        };

        s.apply_config(config);

        s
    }

    fn apply_config(&mut self, config: RPSConfig) {
        let center_x_index = self.info.data1[0].len() / 2 + 1;
        let center_y_index = self.info.data1.len() / 2 + 1;

        self.config = config;

        match config {
            RPSConfig::AllRandom => {
                for y in 0..self.info.data1.len() {
                    for x in 0..self.info.data1[0].len() {
                        self.info.data1[y][x] = match rand::random_range(0..3) {
                            0 => RPSState::Rock,
                            1 => RPSState::Paper,
                            _ => RPSState::Scissors
                        }
                    }
                }
            }
            RPSConfig::RandomCenter(size) => {
                for y in center_y_index-(size/2)..center_y_index+(size/2) {
                    for x in center_x_index-(size/2)..center_x_index+(size/2) {
                        self.info.data1[y][x] = match rand::random_range(0..3) {
                            0 => RPSState::Rock,
                            1 => RPSState::Paper,
                            _ => RPSState::Scissors
                        }
                    }
                }
            }
        }
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
            RPSNeighborhoodType::Moore => {
                vec![
                    (-1, -1),(0, -1),(1, -1),
                    (-1, 0),(1, 0),
                    (-1, 1),(0, 1),(1, 1)
                ]
            }
            RPSNeighborhoodType::VonNeumann => {
                vec![
                    (0, -1),
                    (-1, 0), (1, 0),
                    (0, 1)
                ]
            }
        };

        let needed_count = match self.neighborhood {
            RPSNeighborhoodType::Moore => 5,
            RPSNeighborhoodType::VonNeumann => 2,
        };

        for y in 0..self.info.get_height() {
            for x in 0..self.info.get_width() {
                let mut counts = vec![0, 0, 0, 0];
                for (sx, sy) in to_read.iter() {
                    match self.info.read_from(x as i32 + sx, y as i32 + sy) {
                        RPSState::None      => { counts[0] += 1; }
                        RPSState::Rock      => { counts[1] += 1; }
                        RPSState::Paper     => { counts[2] += 1; }
                        RPSState::Scissors  => { counts[3] += 1; }
                    }

                    match self.info.read_from(x as i32, y as i32) {
                        RPSState::None => {
                            let possibilities = [RPSState::Rock, RPSState::Paper, RPSState::Scissors];

                            let weights = &counts[1..];

                            let dist = WeightedIndex::new(weights).unwrap();
                            let mut rng = rand::rng();

                            self.info.write_data_in_place(x as i32, y as i32, possibilities[dist.sample(&mut rng)]);
                        }
                        RPSState::Rock => {
                            if counts[2] >= needed_count {
                                self.info.write_data_in_place(x as i32, y as i32, RPSState::Paper);
                            }
                        }
                        RPSState::Paper => {
                            if counts[3] >= needed_count {
                                self.info.write_data_in_place(x as i32, y as i32, RPSState::Scissors);
                            }
                        }
                        RPSState::Scissors => {
                            if counts[1] >= needed_count {
                                self.info.write_data_in_place(x as i32, y as i32, RPSState::Rock);
                            }
                        }
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

        if is_key_pressed(KeyCode::Key1) {
            self.editing_cell_state = RPSState::None;
        }
        if is_key_pressed(KeyCode::Key2) {
            self.editing_cell_state = RPSState::Rock;
        }
        if is_key_pressed(KeyCode::Key3) {
            self.editing_cell_state = RPSState::Paper;
        }
        if is_key_pressed(KeyCode::Key4) {
            self.editing_cell_state = RPSState::Scissors;
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
            if is_mouse_button_down(MouseButton::Left) {
                self.info.mouse_set_cell(self.editing_cell_state);
            }
        }
    }

    fn draw(&self) {
        let cell_width = screen_width() / (self.info.seen_range_x.1 - self.info.seen_range_x.0) as f32;
    
        for y in self.info.seen_range_y.0..self.info.seen_range_y.1 {
            for x in self.info.seen_range_x.0..self.info.seen_range_x.1 {
                let x_position = ((x - self.info.seen_range_x.0) as f32) * cell_width;
                let y_position = ((y - self.info.seen_range_y.0) as f32) * cell_width;

                match self.info.read_from(x as i32, y as i32){
                    RPSState::Rock => { draw_rectangle(x_position, y_position, cell_width, cell_width, RED); }
                    RPSState::Paper => { draw_rectangle(x_position, y_position, cell_width, cell_width, GREEN); }
                    RPSState::Scissors => { draw_rectangle(x_position, y_position, cell_width, cell_width, BLUE); }
                    _ => {}
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
            1.0, WHITE
        );
    }
}

impl RPS {
    pub fn set_neighborhood(&mut self, neighborhood: RPSNeighborhoodType) {
        self.neighborhood = neighborhood;
    }
}