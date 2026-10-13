use std::time::Instant;

use macroquad::input::{KeyCode, is_key_down};

pub const DELAYER_DURATION_MILLIS: u128 = 500;

pub struct Delayer {
    is_held: bool,
    delay_time: u128,
    start_time: Instant,
}

impl Delayer {
    pub fn new(delay_time: Option<u128>) -> Self {
        let dtime = match delay_time {
            Some(t) => t,
            None => DELAYER_DURATION_MILLIS
        };

        Self {
            is_held: false,
            delay_time: dtime,
            start_time: Instant::now()
        }
    }

    pub fn run_delayer(&mut self, keys: &[KeyCode]) -> bool {
        let mut status = false;

        let mut at_least_one = false;

        for key in keys {
            if is_key_down(*key) {
                at_least_one = true;
                if !self.is_held {
                    self.is_held = true;
                    self.start_time = Instant::now();
                    return true;
                }

                let elapsed = self.start_time.elapsed().as_millis();
                if elapsed >= self.delay_time {
                    status = true;
                }
            }
        }

        if !at_least_one {
            self.is_held = false;
        }

        status
    }
}