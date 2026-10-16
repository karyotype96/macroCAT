use macroquad::{color::WHITE, text::{draw_text, measure_text}, window::{screen_height, screen_width}};

use crate::{MenuScreen, screens::{button::Button, screen::Screen}, toaster::toaster::{Toaster}};

pub struct Automaton1DScreen;

impl Screen for Automaton1DScreen {
    fn init(&mut self) { }

    fn get_buttons(&self) -> Vec<Button> {
        let button_top = screen_height() / 6.0;

        let button1 = Button::new(
            "Elementary Automaton",
            (screen_width() / 4.0, button_top),
            None,
            None,
        );

        let button2 = Button::new(
            "Extended Automaton",
            (screen_width() / 4.0, button_top + button1.get_dimensions().1 + 20.0),
            None,
            None,
        );

        vec![button1, button2]
    }

    fn control(&mut self, current_screen: &mut MenuScreen, toaster: &mut Toaster) {
        let buttons = self.get_buttons();
        
        if buttons[0].control() {
            toaster.flush();
            toaster.add_message(
                "Hit the checkboxes to change the bits of the automaton rule.", 
                None, None, None, None
            );
            toaster.add_message(
                "Press the button in the middle of the screen to change the starting cell config.", 
                None, None, None, None
            );

            *current_screen = MenuScreen::ElementaryAutomatonOptions;
        }
        if buttons[1].control() {
            toaster.flush();
            toaster.add_message(
                "Hit the checkboxes to change the bits of the automaton rule.", 
                None, None, None, None
            );
            toaster.add_message(
                "Press the button in the middle of the screen to change the starting cell config.", 
                None, None, None, None
            );
            
            *current_screen = MenuScreen::ExtendedAutomatonOptions;
        }
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