use macroquad::{color::{BLUE, WHITE}, text::{draw_text, measure_text}, window::{screen_height, screen_width}};

use crate::{automata::automaton_2d::rps::{RPSConfig, RPSNeighborhoodType}, screens::{button::{self, MenuButton}, screen::Screen}};

pub struct RPSSettingsScreen {
    pub cfg: RPSConfig,
    pub neighborhood: RPSNeighborhoodType,
}

impl Screen<(RPSConfig, RPSNeighborhoodType)> for RPSSettingsScreen {
    fn new() -> Self {
        let mut s = Self {
            cfg: RPSConfig::AllRandom,
            neighborhood: RPSNeighborhoodType::Moore,
        };

        s.init();

        s
    }

    fn init(&mut self) {}

    fn get_buttons(&self) -> Vec<MenuButton> {
        let button_label = match self.cfg {
            RPSConfig::AllRandom => String::from("ALL RANDOM"),
            RPSConfig::RandomCenter(size) => format!("RANDOM CENTER {size}x{size}")
        };

        let neighborhood_label = match self.neighborhood {
            RPSNeighborhoodType::Moore => String::from("MOORE NEIGHBORHOOD"),
            RPSNeighborhoodType::VonNeumann => String::from("VON NEUMANN NEIGHBORHOOD")
        };

        let config_button = MenuButton::new(
            button_label.as_str(),
            (screen_width() / 4.0, screen_height() / 2.0),
            None,
            None
        );

        let neighborhood_button = MenuButton::new(
            neighborhood_label.as_str(),
            (screen_width() * 3.0 / 4.0, screen_height() / 2.0),
            None,
            None
        );

        let start_button = MenuButton::new(
            "START!",
            (screen_width() / 2.0, screen_height() * 7.0 / 8.0),
            None,
            Some(BLUE)
        );

        vec![config_button, neighborhood_button, start_button]
    }

    fn get_info(&self) -> (RPSConfig, RPSNeighborhoodType) {
        (self.cfg, self.neighborhood)
    }

    fn control(&mut self) -> Option<usize> {
        let buttons = self.get_buttons();

        if buttons[0].control() {
            self.cfg = match self.cfg {
                RPSConfig::AllRandom => RPSConfig::RandomCenter(10),
                RPSConfig::RandomCenter(_) => RPSConfig::AllRandom,
            };
        }

        if buttons[1].control() {
            self.neighborhood = match self.neighborhood {
                RPSNeighborhoodType::Moore => RPSNeighborhoodType::VonNeumann,
                RPSNeighborhoodType::VonNeumann => RPSNeighborhoodType::Moore,
            };
        }

        if buttons[2].control() {
            return Some(0);
        }

        None
    }

    fn draw(&self) {
        let buttons = self.get_buttons();

        let label = format!("Rock-Paper-Scissors");
        let label_dimensions = measure_text(
            label.clone(), 
            None, 
            30, 1.0
        );

        draw_text(
            label.clone(),
            screen_width() / 2.0 - label_dimensions.width / 2.0,
            screen_height() / 3.5,
            30.0, WHITE,
        );

        for b in buttons.iter() {
            b.draw();
        }
    }
}