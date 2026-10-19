use macroquad::{color::WHITE, text::{draw_text, measure_text}, window::{screen_height, screen_width}};

use crate::{MenuScreen, screens::{button::Button, screen::Screen}, toaster::toaster::Toaster};

pub struct Automaton2DScreen;

impl Screen for Automaton2DScreen {
    fn init(&mut self) {}

    fn get_buttons(&self) -> Vec<Button> {
        let button_top = screen_height() / 6.0;

        let lifelike_button = Button::new(
            "LIFELIKE AUTOMATON",
            (screen_width() / 4.0, button_top),
            None,
            None
        );

        vec![lifelike_button]
    }

    fn control(&mut self, current_screen: &mut MenuScreen, _toaster: &mut Toaster) {
        let buttons = self.get_buttons();
        
        if buttons[0].control() {
            *current_screen = MenuScreen::LifelikeAutomatonOptions;
        }
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