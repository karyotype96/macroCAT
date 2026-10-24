use crate::{automata::automaton_1d::{base::Automaton1D, elementary::{Elementary, ElementaryConfig}}, screens::{button::MenuButton, screen::Screen}};

pub struct ElementaryShowScreen {
    pub is_init: bool,
    pub el: Elementary,
}

impl Screen<()> for ElementaryShowScreen {
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