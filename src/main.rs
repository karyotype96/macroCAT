pub mod automata;
pub mod helpers;

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

    let mut el = Elementary::new(2000, cfg, 150);

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        el.control();

        if is_key_pressed(KeyCode::R) {
            el.restart(cfg);
        }

        el.draw();

        next_frame().await
    }
}
