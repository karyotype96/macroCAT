pub mod automata;
pub mod helpers;
pub mod toaster;

use macroquad::prelude::*;

use crate::{automata::automaton_1d::{base::Automaton1D, elementary::{Elementary, ElementaryConfig}}, toaster::toaster::{Toaster}};

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

    let mut el = Elementary::new(2000, cfg, 30);
    let mut toaster = Toaster::new();

    toaster.add_message(
        "Press SPACE to advance the automaton.", 
        None, 
        Some(2000), 
        None, 
        None
    );
    toaster.add_message(
        "Press LEFT/A to move the view to the left and RIGHT/D to move it to the right.",
        None,
        None,
        None,
        None
    );
    
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
        toaster.run();

        next_frame().await
    }
}
