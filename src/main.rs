pub mod automata;
pub mod helpers;
pub mod toaster;

use macroquad::prelude::*;
use ::rand::random;

use crate::{automata::automaton_1d::{base::Automaton1D, elementary::{Elementary, ElementaryConfig}, extended::{Extended, ExtendedConfig}}, toaster::toaster::Toaster};

fn window_conf() -> Conf {
    Conf {
        window_title: "MacroCAT".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let cfg = ExtendedConfig::CenterPixel;

    let mut rule: u32 = random();

    let mut el = Extended::new(2000, cfg, rule);
    let mut toaster = Toaster::new();

    toaster.add_message(
        format!("Rule selected: {}", rule).as_str(),
        None,
        None,
        Some(10000),
        None
    );
    
    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::Backspace) {
            toaster.flush();
            rule = random();
            toaster.add_message(
                format!("Rule selected: {}", rule).as_str(),
                None,
                None,
                Some(10000),
                None
            );

            el = Extended::new(2000, cfg, rule);
        }
        if is_key_pressed(KeyCode::M) {
            toaster.flush();
            toaster.add_message(
                format!("Rule selected: {}", rule).as_str(),
                None,
                None,
                Some(10000),
                None
            );
        }

        el.control();
        el.draw();
        toaster.run();

        next_frame().await
    }
}
