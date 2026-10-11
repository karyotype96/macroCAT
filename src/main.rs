use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "MacroCAT".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }
}
