use macroquad::input::{KeyCode, is_key_pressed};

use crate::{MenuScreen, automata::automaton_2d::{base::Automaton2D, brians_brain::{BriansBrain, BriansBrainConfig, DummyRule}}, screens::{button::MenuButton, screen::Screen}, toaster::toaster::Toaster};

pub struct BriansBrainShowScreen {
    pub is_init: bool,
    pub el: BriansBrain,
}

impl Screen for BriansBrainShowScreen {
    fn init(&mut self) {}

    fn get_buttons(&self) -> Vec<MenuButton> {
        vec![]
    }
    
    fn control(&mut self, current_screen: &mut MenuScreen, _toaster: &mut Toaster) {
        self.el.control();

        if is_key_pressed(KeyCode::Q) {
            *current_screen = MenuScreen::BriansBrainOptions;
            self.reset();
        }
    }

    fn draw(&self) {
        self.el.draw();
    }
}

impl BriansBrainShowScreen {
    pub fn new() -> Self {
        Self {
            is_init: false,
            el: BriansBrain::new(1000, 1000, BriansBrainConfig::AllRandom, DummyRule{})
        }
    }

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