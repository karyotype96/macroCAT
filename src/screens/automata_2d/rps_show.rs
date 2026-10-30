use crate::{automata::automaton_2d::{base::Automaton2D, brians_brain::DummyRule, rps::{RPS, RPSConfig, RPSNeighborhoodType}}, screens::{button::MenuButton, screen::Screen}};

pub struct RPSShowScreen {
    pub is_init: bool,
    pub el: RPS,
}

impl Screen<()> for RPSShowScreen {
    fn new() -> Self {
        Self {
            is_init: false,
            el: RPS::new(1000, 1000, RPSConfig::AllRandom, DummyRule)
        }
    }

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

impl RPSShowScreen {
    pub fn set_info(&mut self, cfg: RPSConfig, neighborhood: RPSNeighborhoodType) {
        if !self.is_init {
            self.el = RPS::new(1000, 1000, cfg, DummyRule);
            self.el.set_neighborhood(neighborhood);
            self.is_init = true;
        }
    }

    pub fn reset(&mut self) {
        self.is_init = false;
    }
}