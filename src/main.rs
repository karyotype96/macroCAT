pub mod automata;

use std::time::Instant;

use macroquad::prelude::*;

use crate::automata::automaton_1d::{base::Automaton1D, elementary::{Elementary, ElementaryConfig}};

fn window_conf() -> Conf {
    Conf {
        window_title: "MacroCAT".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let cfg = ElementaryConfig::CenterPixel;

    let mut is_held = false;
    let mut start_time = Instant::now();
    let mut el = Elementary::new(2000, cfg, 29);

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_down(KeyCode::Space) {
            if !is_held {
                is_held = true;
                start_time = Instant::now();
            }

            if start_time.elapsed().as_millis() >= 33 {
                el.pass_time();
                start_time = Instant::now();
            }
        }
        if is_key_released(KeyCode::Space) {
            is_held = false;
        }

        if is_key_pressed(KeyCode::R) {
            el.restart(cfg);
        }

        el.draw();

        next_frame().await
    }
}
