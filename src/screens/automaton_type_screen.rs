use macroquad::{color::WHITE, text::{draw_text, measure_text}, window::{screen_height, screen_width}};

use crate::{MenuScreen, screens::{button::{Button}, screen::Screen}};

pub fn set_to_automata_1d(current_screen: &mut MenuScreen) {
    *current_screen = MenuScreen::SelectAutomaton1D;
}

pub struct AutomatonTypeScreen;

impl Screen for AutomatonTypeScreen {
    fn get_buttons(&self) -> Vec<Button> {
        let button1 = Button::new(
            "1D Automata",
            (screen_width() / 2.0, screen_height() / 3.0),
            None,
            None,
            |s: &mut MenuScreen| { *s = MenuScreen::SelectAutomaton1D }
        );

        let button2 = Button::new(
            "2D Automata",
            (screen_width() / 2.0, screen_height() / 3.0 + button1.get_dimensions().1 + 20.0),
            None,
            None,
            |s: &mut MenuScreen| { *s = MenuScreen::SelectAutomaton2D }
        );

        let button3 = Button::new(
            "3D Automata",
            (screen_width() / 2.0, screen_height() / 3.0 + button1.get_dimensions().1 + button2.get_dimensions().1 + 40.0),
            None,
            None,
            |s: &mut MenuScreen| { *s = MenuScreen::SelectAutomaton3D }
        );

        vec![button1, button2, button3]
    }

    fn control(&self, current_screen: &mut MenuScreen) {
        let buttons = self.get_buttons();
        for i in 0..buttons.len() {
            buttons[i].control(current_screen);
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