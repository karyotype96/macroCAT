use macroquad::{color::{BLACK, BLUE, GREEN, WHITE}, input::{KeyCode, is_key_pressed}, shapes::{draw_rectangle, draw_rectangle_lines}, text::{draw_text, measure_text}, window::{screen_height, screen_width}};
use rand::random;

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

        let config_button = Button::new(
            button_label.as_str(),
            (screen_width() / 2.0, screen_height() * 3.0 / 4.0),
            None,
            None
        );

        let start_button = Button::new(
            "START!",
            (screen_width() / 2.0, screen_height() * 7.0 / 8.0),
            None,
            Some(BLUE),
        );

        let random_rule_button = Button::new(
            "Select random rule",
            (screen_width() / 2.0, screen_height() / 2.0),
            None,
            None
        );

        vec![config_button, start_button, random_rule_button]
    }


    fn control(&mut self, current_screen: &mut MenuScreen, toaster: &mut Toaster) {
        if is_key_pressed(KeyCode::Q) {
            toaster.flush();
            *current_screen = MenuScreen::SelectAutomaton1D;
        }
        
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
            toaster.add_message(
                "Press SPACE to advance time, and LEFT/RIGHT or A/D to move the view.", 
                None, None, None, None
            );
            toaster.add_message(
                "Press R to reset the automaton, and Q to go back to settings.", 
                None, None, None, None
            );
            *current_screen = MenuScreen::ElementaryAutomatonShow(self.cfg, self.rule);
        }

        if buttons[2].control() {
            self.random_rule();
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
            self.draw_rule(&self.checkboxes[i], i as u8);
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

    fn random_rule(&mut self) {
        let rule: u8 = random();

        self.rule = rule;
        for i in 0..self.checkboxes.len() {
            let bit_index = 7-i;

            if self.rule & (1 << bit_index) > 0 {
                self.checkboxes[i].set(true);
            } else {
                self.checkboxes[i].set(false);
            }
        }
    }

    // draw a small window concisely explaining the rule
    // based on each checkbox
    fn draw_rule(&self, checkbox: &Checkbox, index: u8) {
        let bit = 7 - index;

        let space_offset: f32 = 10.0;
        let width: f32 = Checkbox::size() * 3.0 + (space_offset * 2.0);
        let height: f32 = Checkbox::size() * 3.0;

        let position = checkbox.position();

        let top_left = (
            position.0 - (width / 2.0),
            position.1 - height - Checkbox::size() / 2.0
        );

        checkbox.draw();

        if checkbox.mouse_over_box() {
            draw_rectangle(
                top_left.0,
                top_left.1,
                width, 
                height,
                BLACK
            );
            draw_rectangle_lines(
                top_left.0, 
                top_left.1, 
                width, 
                height,
                2.0,
                WHITE
            );

            for i in (0..3).rev() {
                let box_pos = (
                    top_left.0 + space_offset + Checkbox::size() / 2.0 + (2 - i) as f32 * Checkbox::size(),
                    top_left.1 + (Checkbox::size())
                );

                if bit & (1 << i) > 0 {
                    draw_rectangle(
                        box_pos.0 - Checkbox::size() / 2.0,
                        box_pos.1 - Checkbox::size() / 2.0,
                        Checkbox::size(), 
                        Checkbox::size(), 
                        GREEN
                    );
                }

                draw_rectangle_lines(
                    box_pos.0 - Checkbox::size() / 2.0,
                    box_pos.1 - Checkbox::size() / 2.0,
                    Checkbox::size(), 
                    Checkbox::size(), 
                    2.0, 
                    WHITE
                );
            }

            if checkbox.checked() {
                draw_rectangle(
                    checkbox.position().0 - Checkbox::size() / 2.0, 
                    top_left.1 + Checkbox::size() * 1.5, 
                    Checkbox::size(), 
                    Checkbox::size(), 
                    WHITE
                );
            } else {
                draw_rectangle_lines(
                    checkbox.position().0 - Checkbox::size() / 2.0, 
                    top_left.1 + Checkbox::size() * 1.5, 
                    Checkbox::size(), 
                    Checkbox::size(), 
                    2.0, 
                    WHITE
                );
            }
        }
    }
}