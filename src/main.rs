pub mod automata;
pub mod helpers;
pub mod toaster;
pub mod screens;

use macroquad::prelude::*;

use crate::screens::nav::Nav;

fn window_conf() -> Conf {
    Conf {
        window_title: "MacroCAT".to_owned(),
        fullscreen: false,
        window_width: 1920,
        window_height: 1080,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    next_frame().await;

    let mut nav = Nav::new();

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        nav.control();
        nav.draw();

        next_frame().await
    }
}
