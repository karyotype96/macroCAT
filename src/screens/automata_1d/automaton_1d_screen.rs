use macroquad::{color::WHITE, text::{draw_text, measure_text}, window::{screen_height, screen_width}};

use crate::{screens::{button::MenuButton, screen::Screen}};

pub struct Automaton1DScreen;

impl Screen<()> for Automaton1DScreen {
    fn new() -> Self {
        Self
    }
    
    fn init(&mut self) { }

    fn get_buttons(&self) -> Vec<MenuButton> {
        let mut button_top = screen_height() / 6.0;

        let elementary_button = MenuButton::new(
            "ELEMENTARY AUTOMATON",
            (screen_width() / 4.0, button_top),
            None,
            None,
        );

        button_top += elementary_button.get_dimensions().1 + 20.0;

        let extended_button = MenuButton::new(
            "EXTENDED AUTOMATON",
            (screen_width() / 4.0, button_top),
            None,
            None,
        );

        button_top += extended_button.get_dimensions().1 + 20.0;

        let totalistic_button = MenuButton::new(
            "TOTALISTIC AUTOMATON",
            (screen_width() / 4.0, button_top),
            None,
            None
        );

        vec![elementary_button, extended_button, totalistic_button]
    }

    fn get_info(&self) {

    }

    fn control(&mut self) -> Option<usize> {
        let buttons = self.get_buttons();

        for i in 0..buttons.len() {
            if buttons[i].control() {
                return Some(i);
            }
        }

        None
    }

    fn draw(&self) {
        let top_label = String::from("Which 1D automaton will you be testing?");
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