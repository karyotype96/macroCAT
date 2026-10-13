use std::time::Instant;

use macroquad::input::{KeyCode, is_key_down};

pub const TIMER_DURATION_MILLIS: u128 = 33;

pub struct Timer {
    is_held: bool,
    start_time: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            is_held: false,
            start_time: Instant::now()
        }
    }

    pub fn run_timer(&mut self, keys: &[KeyCode]) -> bool {
        let mut status = false;

        let mut at_least_one = false;
        for key in keys {
            if is_key_down(*key) {
                at_least_one = true;
                if !self.is_held {
                    self.is_held = true;
                    self.start_time = Instant::now();
                }

                if self.start_time.elapsed().as_millis() >= TIMER_DURATION_MILLIS {
                    status = true;
                    self.start_time = Instant::now();
                }
            }
        }

        if !at_least_one {
            self.is_held = false;
        }

        status
    }
}