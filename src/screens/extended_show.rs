use crate::{automata::automaton_1d::{base::{Automaton1D}, extended::{Extended, ExtendedConfig}}, screens::{button::MenuButton, screen::Screen}};

pub struct ExtendedShowScreen {
    pub is_init: bool,
    pub el: Extended,
}

impl Screen<()> for ExtendedShowScreen {
    fn init(&mut self) {}

    fn get_buttons(&self) -> Vec<MenuButton> {
        vec![]
    }

    fn get_info(&self) {}

    fn control(&mut self) -> Option<usize> {
        self.el.control();

        None
    }

    fn draw(&self) {
        self.el.draw();
    }
}

impl ExtendedShowScreen {
    pub fn new() -> Self {
        Self {
            is_init: false,
            el: Extended::new(2000, ExtendedConfig::CenterPixel, 0)
        }
    }

    pub fn set_info(&mut self, cfg: ExtendedConfig, rule: u32) {
        if !self.is_init {
            self.el = Extended::new(2000, cfg, rule);
            self.is_init = true;
        }
    }

    pub fn reset(&mut self) {
        self.is_init = false;
    }
}