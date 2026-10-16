use macroquad::{color::{BLUE, WHITE}, text::{draw_text, measure_text}, window::{screen_height, screen_width}};

use crate::{MenuScreen, automata::automaton_1d::elementary::ElementaryConfig, screens::{button::Button, checkbox::Checkbox, screen::Screen}, toaster::toaster::Toaster};

pub struct ElementarySettingsScreen {
    pub checkboxes: Vec<Checkbox>,

    pub cfg: ElementaryConfig,
    pub rule: u8
}

impl Screen for ElementarySettingsScreen {
    fn init(&mut self) {
        let center_x = screen_width() / 2.0;
        let leftmost_x = center_x - Checkbox::size()*3.5;

        for i in 0..8 {
            self.checkboxes.push(Checkbox::new(
                None,
                (leftmost_x + Checkbox::size()*(i as f32), screen_height() / 3.0)
            ));
        };
    }

    fn get_buttons(&self) -> Vec<Button> {
        let button_label = match self.cfg {
            ElementaryConfig::CenterPixel => String::from("Center Pixel"),
            ElementaryConfig::AllRandom => String::from("All Random"),
            ElementaryConfig::RandomCenter(n) => format!("Random Center {}", n*2-1)
        };

        let button1 = Button::new(
            button_label.as_str(),
            (screen_width() / 2.0, screen_height() / 2.0),
            None,
            None
        );

        let button2 = Button::new(
            "START!",
            (screen_width() / 2.0, screen_height() * 7.0 / 8.0),
            None,
            Some(BLUE),
        );

        vec![button1, button2]
    }


    fn control(&mut self, current_screen: &mut MenuScreen, toaster: &mut Toaster) {
        let buttons = self.get_buttons();

        if buttons[0].control() {
            self.cfg = match self.cfg {
                ElementaryConfig::CenterPixel => ElementaryConfig::AllRandom,
                ElementaryConfig::AllRandom => ElementaryConfig::RandomCenter(3),
                ElementaryConfig::RandomCenter(_) => ElementaryConfig::CenterPixel
            };
        }

        if buttons[1].control() {
            toaster.flush();
            toaster.add_message(
                format!("Rule: {}", self.rule).as_str(), 
                None, None, None, None
            );
            *current_screen = MenuScreen::ElementaryAutomatonShow(self.cfg, self.rule);
        }

        for i in 0..self.checkboxes.len() {
            if self.checkboxes[i].control() {
                let bit: u8 = 7 - i as u8;
                self.flip_bit(bit, self.checkboxes[i].checked());
            }
        }
    }

    fn draw(&self) {
        let buttons = self.get_buttons();

        let label = format!("Rule Selected: {}", self.rule);

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

        for i in 0..self.checkboxes.len() {
            self.checkboxes[i].draw();
        }
    }
}

impl ElementarySettingsScreen {
    fn flip_bit(&mut self, bit: u8, is_one: bool) {
        if bit >= 8 { return; }

        if is_one {
            self.rule |= 1 << bit;
        } else {
            self.rule &= !(1 << bit);
        }
    }
}