use crate::{automata::automaton_2d::{base::Automaton2D, brians_brain::{BriansBrain, BriansBrainConfig, DummyRule}}, screens::{button::MenuButton, screen::Screen}};

pub struct BriansBrainShowScreen {
    pub is_init: bool,
    pub el: BriansBrain,
}

impl Screen<()> for BriansBrainShowScreen {
    fn new() -> Self {
        Self {
            is_init: false,
            el: BriansBrain::new(1000, 1000, BriansBrainConfig::AllRandom, DummyRule)
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

impl BriansBrainShowScreen {
    pub fn set_info(&mut self, cfg: BriansBrainConfig) {
        if !self.is_init {
            self.el = BriansBrain::new(1000, 1000, cfg, DummyRule{});
            self.is_init = true;
        }
    }

    pub fn reset(&mut self) {
        self.is_init = false;
    }
}