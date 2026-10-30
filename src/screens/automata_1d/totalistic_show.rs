use crate::{automata::automaton_1d::{base::Automaton1D, totalistic::{Totalistic, TotalisticConfig}}, screens::{button::MenuButton, screen::Screen}};

pub struct TotalisticShowScreen {
    pub is_init: bool,
    pub el: Totalistic,
}

impl Screen<()> for TotalisticShowScreen {
    fn new() -> Self {
        Self {
            is_init: false,
            el: Totalistic::new(200, TotalisticConfig::CenterPixel, 0)
        }
    }

    fn init(&mut self) {}

    fn get_buttons(&self) -> Vec<MenuButton> {
        vec![]
    }

    fn get_info(&self) { }

    fn control(&mut self) -> Option<usize> {
        self.el.control();

        None
    }

    fn draw(&self) {
        self.el.draw();
    }
}

impl TotalisticShowScreen {
    pub fn set_info(&mut self, cfg: TotalisticConfig, rule: u16) {
        if !self.is_init {
            self.el = Totalistic::new(2000, cfg, rule);
            self.is_init = true;
        }
    }

    pub fn reset(&mut self) {
        self.is_init = false;
    }
}