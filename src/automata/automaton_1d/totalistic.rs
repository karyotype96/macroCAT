use macroquad::{color::{BLACK, GRAY, WHITE}, input::{KeyCode, is_key_down, is_key_pressed}, shapes::draw_rectangle, window::screen_width};

use crate::{automata::automaton_1d::base::{Automaton1D, Automaton1DInfo}, helpers::{delayer::Delayer, timer::Timer}};

#[derive(Copy, Clone)]
pub enum TotalisticConfig {
    CenterPixel,
    AllRandom,
    RandomCenter(usize)
}

pub struct Totalistic {
    pub info: Automaton1DInfo<u8>,
    pub config: TotalisticConfig,
    pub rule: u16,
    pub timer: Timer,
    pub advance_delayer: Delayer,
    pub move_delayer: Delayer,
}

impl Automaton1D<u8, TotalisticConfig, u16> for Totalistic {
    fn new(width: usize, config: TotalisticConfig, rule: u16) -> Self {
        if width < 100 {
            panic!("Automaton should not have a width less than 100.");
        }

        let info: Automaton1DInfo<u8> = Automaton1DInfo::new(width, (width / 2 + 1 - 50, width / 2 + 1 + 50));

        let mut s = Self {
            info,
            config,
            rule,
            timer: Timer::new(),
            advance_delayer: Delayer::new(None),
            move_delayer: Delayer::new(None)
        };

        s.apply_config(config);

        s
    }

    fn apply_config(&mut self, config: TotalisticConfig) {
        let center_index = self.info.data[0].len() / 2 + 1;

        self.config = config;

        match config {
            TotalisticConfig::CenterPixel => {
                self.info.data[0][center_index] = 2;
            }
            TotalisticConfig::AllRandom => {
                for i in 0..self.info.data[0].len() {
                    self.info.data[0][i] = rand::random_range(0..3);
                }
            }
            TotalisticConfig::RandomCenter(size) => {
                for i in center_index-size..center_index+size {
                    self.info.data[0][i] = rand::random_range(0..3);
                }
            }
        }
    }

    fn restart(&mut self) {
        let width = self.info.data[0].len();
        let seen_range = (width / 2 + 1 - 50, width / 2 + 1 + 50);

        self.info = Automaton1DInfo::new(width, seen_range);
        self.apply_config(self.config);
    }

    fn pass_time(&mut self) {
        self.info.increment_time();

        let rule = self.convert_rule();

        let previous_row = self.info.get_previous_row();
        for i in 0..self.info.get_width() {
            let mut total = 0;
            if i > 0 {
                total += previous_row[i-1];
            }
            total += previous_row[i];
            if i < previous_row.len()-1 {
                total += previous_row[i+1];
            }

            self.info.data[self.info.current_row][i] = rule[total as usize];
        }
    }

    fn move_window(&mut self, direction: i32) {
        self.info.move_window(direction);
    }

    fn control(&mut self) {
        if self.advance_delayer.run_delayer(&[KeyCode::Space]) && self.timer.run_timer(&[KeyCode::Space]) {
            self.pass_time();
        }

        let move_keys = [KeyCode::Left, KeyCode::Right, KeyCode::A, KeyCode::D];
        if self.move_delayer.run_delayer(&move_keys) && self.timer.run_timer(&move_keys) {
            let mut movement = 0;
            if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                movement -= 1;
            }
            if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                movement += 1;
            }

            self.move_window(movement);
        }

        if is_key_pressed(KeyCode::R) {
            self.restart();
        }
    }

    fn draw(&self) {
        let cell_width = screen_width() / (self.info.seen_range.1 - self.info.seen_range.0) as f32;

        for y in 0..self.info.get_height() {
            for x in self.info.seen_range.0..self.info.seen_range.1 {
                let x_position = ((x - self.info.seen_range.0) as f32) * cell_width;
                let y_position = (y as f32) * cell_width;

                let cell_color = match self.info.data[y][x] {
                    2 => WHITE,
                    1 => GRAY,
                    _ => BLACK,
                };

                if self.info.data[y][x] != 0 {
                    draw_rectangle(x_position, y_position, cell_width, cell_width, cell_color);
                }
            }
        }
    }
}

impl Totalistic {
    fn convert_rule(&self) -> Vec<u8> {
        let mut rule_vec: Vec<u8> = Vec::new();
        let mut temp_rule = self.rule;

        for _ in 0..7 {
            let val: u8 = (temp_rule % 3) as u8;
            rule_vec.push(val);
            temp_rule /= 3;
        }

        rule_vec
    }
}