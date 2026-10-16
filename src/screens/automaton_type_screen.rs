use macroquad::{color::WHITE, text::{draw_text, measure_text}, window::{screen_height, screen_width}};

use crate::{MenuScreen, screens::{button::Button, screen::Screen}, toaster::toaster::Toaster};

pub struct AutomatonTypeScreen;

impl Screen for AutomatonTypeScreen {
    fn init(&mut self) { }

    fn get_buttons(&self) -> Vec<Button> {
        let button1 = Button::new(
            "1D Automata",
            (screen_width() / 2.0, screen_height() / 3.0),
            None,
            None,
        );

        let button2 = Button::new(
            "2D Automata",
            (screen_width() / 2.0, screen_height() / 3.0 + button1.get_dimensions().1 + 20.0),
            None,
            None,
        );

        let button3 = Button::new(
            "3D Automata",
            (screen_width() / 2.0, screen_height() / 3.0 + button1.get_dimensions().1 + button2.get_dimensions().1 + 40.0),
            None,
            None,
        );

        vec![button1, button2, button3]
    }

    fn control(&mut self, current_screen: &mut MenuScreen, toaster: &mut Toaster) {
        let buttons = self.get_buttons();
        
        if buttons[0].control() {
            toaster.flush();
            toaster.add_message(
                "Select a 1-dimensional automaton.", 
                None, None, None, None
            );
            *current_screen = MenuScreen::SelectAutomaton1D;
        }
        if buttons[1].control() {
            toaster.flush();
            toaster.add_message(
                "Select a 2-dimensional automaton.", 
                None, None, None, None
            );
            *current_screen = MenuScreen::SelectAutomaton2D;
        }
        if buttons[2].control() {
            toaster.flush();
            toaster.add_message(
                "Select a 3-dimensional automaton.", 
                None, None, None, None
            );
            *current_screen = MenuScreen::SelectAutomaton3D;
        }
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