use macroquad::input::{KeyCode, is_key_pressed};

use crate::{MenuScreen, automata::automaton_2d::{base::Automaton2D, lifelike::{Lifelike, LifelikeConfig, LifelikeNeighborhoodType}}, screens::{button::MenuButton, screen::Screen}, toaster::toaster::Toaster};

pub struct LifelikeShowScreen {
    pub is_init: bool,
    pub el: Lifelike,
}

impl Screen for LifelikeShowScreen {
    fn init(&mut self) {}

    fn get_buttons(&self) -> Vec<MenuButton> {
        vec![]
    }

    fn control(&mut self, current_screen: &mut MenuScreen, _toaster: &mut Toaster) {
        self.el.control();

        if is_key_pressed(KeyCode::Q) {
            *current_screen = MenuScreen::LifelikeAutomatonOptions;
            self.reset();
        }
    }

    fn draw(&self) {
        self.el.draw();
    }
}

impl LifelikeShowScreen {
    pub fn new() -> Self {
        Self {
            is_init: false,
            el: Lifelike::new(1000, 1000, LifelikeConfig::None, (0, 0))
        }
    }

    pub fn set_info(&mut self, cfg: LifelikeConfig, neighborhood: LifelikeNeighborhoodType, rule: (u16, u16)) {
        if !self.is_init {
            self.el = Lifelike::new(1000, 1000, cfg, rule);
            self.el.set_neighborhood(neighborhood);
            self.is_init = true;
        }
    }

    pub fn reset(&mut self) {
        self.is_init = false;
    }
}