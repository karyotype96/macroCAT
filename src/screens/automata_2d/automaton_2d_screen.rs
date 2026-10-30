use macroquad::{color::WHITE, text::{draw_text, measure_text}, window::{screen_height, screen_width}};

use crate::{screens::{button::{MenuButton}, screen::Screen}};

pub struct Automaton2DScreen;

impl Screen<()> for Automaton2DScreen {
    fn new() -> Self {
        Self
    }

    fn init(&mut self) {}

    fn get_buttons(&self) -> Vec<MenuButton> {
        let mut button_top = screen_height() / 6.0;

        let lifelike_button = MenuButton::new(
            "LIFELIKE AUTOMATON",
            (screen_width() / 4.0, button_top),
            None,
            None
        );

        button_top += lifelike_button.get_dimensions().1 + 20.0;
        let brians_brain_button = MenuButton::new(
            "BRIAN'S BRAIN",
            (screen_width() / 4.0, button_top),
            None,
            None
        );

        button_top += brians_brain_button.get_dimensions().1 + 20.0;
        let rps_button = MenuButton::new(
            "ROCK PAPER SCISSORS",
            (screen_width() / 4.0, button_top),
            None,
            None
        );

        vec![lifelike_button, brians_brain_button, rps_button]
    }

    fn get_info(&self) {}

    fn control(&mut self) -> Option<usize> {
        let buttons = self.get_buttons();
        
        for i in 0..buttons.len() {
            if buttons[i].control() {
                return Some(i);
            }
        }

        return None;
    }

    fn draw(&self) {
        let top_label = String::from("Which 2D automaton will you be testing?");
        let dimensions = measure_text(
            top_label.as_str(),
            None, 
            30, 
            1.0
        );

        draw_text(top_label.as_str(),
            screen_width() / 2.0 - dimensions.width / 2.0,
            screen_height() / 8.0,
            30.0, WHITE
        );

        let buttons = self.get_buttons();
        for i in 0..buttons.len() {
            buttons[i].draw();
        }
    }
}