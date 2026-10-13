use crate::{automata::automaton_1d::base::{Automaton1D, Automaton1DInfo}, helpers::{delayer::Delayer, timer::Timer}};

use macroquad::{color::WHITE, input::KeyCode, shapes::draw_rectangle, window::screen_width};
use rand;

#[derive(Copy, Clone)]
pub enum ElementaryConfig {
    CenterPixel,
    AllRandom,
    RandomCenter(usize)
}

pub struct Elementary {
    pub info: Automaton1DInfo<bool>,
    pub rule: u8,
    pub timer: Timer,
    pub advance_delayer: Delayer,
}

impl Automaton1D<bool, ElementaryConfig, u8> for Elementary {
    fn new(width: usize, config: ElementaryConfig, rule: u8) -> Self {
        if width < 100 {
            panic!("Automaton should not have a width less than 100");
        }

        let info: Automaton1DInfo<bool> = Automaton1DInfo::new(width, (width / 2 + 1 - 50, width / 2 + 1 + 50));
        
        let mut s = Self {
            info,
            rule,
            timer: Timer::new(),
            advance_delayer: Delayer::new(None),
        };

        s.apply_config(config);

        s
    }

    fn apply_config(&mut self, config: ElementaryConfig) {
        let center_index = self.info.data[0].len() / 2 + 1;

        match config {
            ElementaryConfig::CenterPixel => {
                self.info.data[0][center_index] = true;
            }
            ElementaryConfig::AllRandom => {
                for i in 0..self.info.data[0].len() {
                    self.info.data[0][i] = rand::random_bool(0.5);
                }
            }
            ElementaryConfig::RandomCenter(size) => {
                for i in center_index-size..center_index+size {
                    self.info.data[0][i] = rand::random_bool(0.5);
                }
            }
        }
    }

    fn restart(&mut self, config: ElementaryConfig) {
        let width = self.info.data[0].len();
        let seen_range = self.info.seen_range;

        self.info = Automaton1DInfo::new(width, seen_range);
        self.apply_config(config);
    }

    fn pass_time(&mut self) {
        self.info.increment_time();

        let mut cell_rules: Vec<(bool, bool, bool)> = Vec::new();
        for i in 0..8 {
            let set = (i & 0b100 > 0, i & 0b010 > 0, i & 0b001 > 0);

            if self.rule & 1 << i > 0 {
                cell_rules.push(set);
            }
        }

        let previous_row = self.info.get_previous_row();
        for i in 0..self.info.get_width() {
            self.info.data[self.info.current_row][i] = false;

            let mut neighbors = (false, false, false);
            if i > 0 {
                neighbors.0 = previous_row[i-1];
            }
            neighbors.1 = previous_row[i];
            if i < previous_row.len()-1 {
                neighbors.2 = previous_row[i+1];
            }

            if cell_rules.contains(&neighbors) {
                self.info.data[self.info.current_row][i] = true;
            }
        }
    }

    fn control(&mut self) {
        if self.advance_delayer.run_delayer(KeyCode::Space) && self.timer.run_timer(KeyCode::Space) {
            self.pass_time();
        }
    }

    fn draw(&self) {
        let cell_width = screen_width() / (self.info.seen_range.1 - self.info.seen_range.0) as f32;

        for y in 0..self.info.get_height() {
            for x in self.info.seen_range.0..self.info.seen_range.1 {
                let x_position = ((x - self.info.seen_range.0) as f32) * cell_width;
                let y_position = (y as f32) * cell_width;

                if self.info.data[y][x] {
                    draw_rectangle(x_position, y_position, cell_width, cell_width, WHITE);
                }
            }
        }
    }
}