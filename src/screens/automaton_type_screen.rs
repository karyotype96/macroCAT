use macroquad::{color::WHITE, text::{draw_text, measure_text}, window::{screen_height, screen_width}};

use crate::{screens::{button::MenuButton, screen::Screen}};

pub struct AutomatonTypeScreen;

impl Screen<()> for AutomatonTypeScreen {
    fn init(&mut self) { }

    fn get_buttons(&self) -> Vec<MenuButton> {
        let button1 = MenuButton::new(
            "1D Automata",
            (screen_width() / 2.0, screen_height() / 3.0),
            None,
            None,
        );

        let button2 = MenuButton::new(
            "2D Automata",
            (screen_width() / 2.0, screen_height() / 3.0 + button1.get_dimensions().1 + 20.0),
            None,
            None,
        );

        let button3 = MenuButton::new(
            "3D Automata",
            (screen_width() / 2.0, screen_height() / 3.0 + button1.get_dimensions().1 + button2.get_dimensions().1 + 40.0),
            None,
            None,
        );

        vec![button1, button2, button3]
    }

    fn get_info(&self) {}

    fn control(&mut self) -> Option<usize> {
        let buttons = self.get_buttons();
        
        if buttons[0].control() {
            return Some(0);
        }
        if buttons[1].control() {
            return Some(1);
        }
        if buttons[2].control() {
            return None;
        }

        None
    }

    fn draw(&self) {
        let top_label = String::from("What type of automaton would you like to use?");
        let dimensions = measure_text(
            top_label.as_str(),
            None,
            30,
            1.0
        );

        draw_text(top_label.as_str(), 
            screen_width() / 2.0 - dimensions.width,
            screen_height() / 6.0, 
            30.0, WHITE);

        let buttons = self.get_buttons();
        for i in 0..buttons.len() {
            buttons[i].draw();
        }
    }
}