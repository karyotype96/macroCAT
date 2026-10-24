use macroquad::{color::{BLUE, WHITE}, text::{draw_text, measure_text}, window::{screen_height, screen_width}};

use crate::{automata::automaton_2d::{brians_brain::BriansBrainConfig}, screens::{button::MenuButton, screen::Screen}};

pub struct BriansBrainSettingsScreen {
    pub cfg: BriansBrainConfig,
}

impl Screen<BriansBrainConfig> for BriansBrainSettingsScreen {
    fn init(&mut self) {}

    fn get_buttons(&self) -> Vec<MenuButton> {
        let button_label = match self.cfg {
            BriansBrainConfig::AllRandom => String::from("ALL RANDOM"),
            BriansBrainConfig::RandomCenter(size) => format!("RANDOM CENTER {size}x{size}")
        };

        let config_button = MenuButton::new(
            button_label.as_str(),
            (screen_width() / 2.0, screen_height() / 2.0),
            None,
            None
        );

        let start_button = MenuButton::new(
            "START!",
            (screen_width() / 2.0, screen_height() * 7.0 / 8.0),
            None,
            Some(BLUE)
        );

        vec![config_button, start_button]
    }

    fn get_info(&self) -> BriansBrainConfig {
        self.cfg
    }

    fn control(&mut self) -> Option<usize> {
        let buttons = self.get_buttons();

        if buttons[0].control() {
            self.cfg = match self.cfg {
                BriansBrainConfig::AllRandom => BriansBrainConfig::RandomCenter(10),
                BriansBrainConfig::RandomCenter(_) => BriansBrainConfig::AllRandom
            };
        }

        if buttons[1].control() {
            return Some(0);
        }

        return None;
    }

    fn draw(&self) {
        let buttons = self.get_buttons();

        let label = format!("Brian's Brain");
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

impl BriansBrainSettingsScreen {
    pub fn new() -> Self {
        let mut s = Self {
            cfg: BriansBrainConfig::AllRandom
        };

        s.init();

        s
    }
}