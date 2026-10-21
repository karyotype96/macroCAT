use macroquad::input::{KeyCode, is_key_pressed};

use crate::{MenuScreen, automata::automaton_1d::{base::Automaton1D, elementary::{Elementary, ElementaryConfig}}, screens::{button::MenuButton, screen::Screen}, toaster::toaster::Toaster};

pub struct ElementaryShowScreen {
    pub is_init: bool,
    pub el: Elementary,
}

impl Screen for ElementaryShowScreen {
    fn init(&mut self) {}

    fn get_buttons(&self) -> Vec<MenuButton> {
        vec![]
    }

    fn control(&mut self, current_screen: &mut MenuScreen, _toaster: &mut Toaster) {
        self.el.control();

        if is_key_pressed(KeyCode::Q) {
            *current_screen = MenuScreen::ElementaryAutomatonOptions;
            self.reset();
        }
    }

    fn draw(&self) {
        self.el.draw();
    }
}

impl ElementaryShowScreen {
    pub fn new() -> Self {
        Self {
            is_init: false,
            el: Elementary::new(2000, ElementaryConfig::CenterPixel, 0)
        }
    }

    pub fn set_info(&mut self, cfg: ElementaryConfig, rule: u8) {
        if !self.is_init {
            self.el = Elementary::new(2000, cfg, rule);
            self.is_init = true;
        }
    }

    pub fn reset(&mut self) {
        self.is_init = false;
    }
}