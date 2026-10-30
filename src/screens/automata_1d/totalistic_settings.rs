use macroquad::{color::{BLACK, BLUE, Color, GRAY, WHITE}, shapes::{draw_rectangle, draw_rectangle_lines}, text::{draw_text, measure_text}, window::{screen_height, screen_width}};

use crate::{automata::automaton_1d::totalistic::TotalisticConfig, screens::{button::MenuButton, number_clickbox::NumberClickbox, screen::Screen}};

pub struct TotalisticSettingsScreen {
    pub clickboxes: Vec<NumberClickbox>,

    pub cfg: TotalisticConfig,
    pub rule: u16
}

impl Screen<(TotalisticConfig, u16)> for TotalisticSettingsScreen {
    fn new() -> Self {
        let mut s = Self {
            clickboxes: Vec::new(),

            cfg: TotalisticConfig::CenterPixel,
            rule: 0
        };

        s.init();

        s
    }

    fn init(&mut self) {
        let center_x = screen_width() / 2.0;
        let leftmost_x = center_x - NumberClickbox::size()*3.0;

        for i in 0..7 {
            self.clickboxes.push(NumberClickbox::new(
                None,
                (leftmost_x + NumberClickbox::size()*(i as f32), screen_height() / 3.0),
                3
            ));
        }
    }

    fn get_buttons(&self) -> Vec<MenuButton> {
        let button_label = match self.cfg {
            TotalisticConfig::CenterPixel => String::from("CENTER PIXEL"),
            TotalisticConfig::AllRandom => String::from("ALL RANDOM"),
            TotalisticConfig::RandomCenter(n) => format!("RANDOM CENTER {}", n*2-1),
        };

        let config_button = MenuButton::new(
            button_label.as_str(),
            (screen_width() / 2.0, screen_height() * 3.0 / 4.0),
            None,
            None
        );

        let start_button = MenuButton::new(
            "START!",
            (screen_width() / 2.0, screen_height() * 7.0 / 8.0),
            None,
            Some(BLUE),
        );

        let random_rule_button = MenuButton::new(
            "Select random rule",
            (screen_width() / 2.0, screen_height() / 2.0),
            None,
            None
        );

        vec![config_button, start_button, random_rule_button]
    }

    fn get_info(&self) -> (TotalisticConfig, u16) {
        (self.cfg, self.rule)
    }

    fn control(&mut self) -> Option<usize> {
        let buttons = self.get_buttons();

        if buttons[0].control() {
            self.cfg = match self.cfg {
                TotalisticConfig::CenterPixel => TotalisticConfig::AllRandom,
                TotalisticConfig::AllRandom => TotalisticConfig::RandomCenter(3),
                TotalisticConfig::RandomCenter(_) => TotalisticConfig::CenterPixel,
            };
        }

        if buttons[1].control() {
            return Some(0);
        }

        if buttons[2].control() {
            self.random_rule();
        }

        for i in 0..self.clickboxes.len() {
            if self.clickboxes[i].control() {
                let two_power: usize = 6-i;
                let mut rule_vec = TotalisticSettingsScreen::rule_code_to_vec(self.rule);

                rule_vec[two_power] = (rule_vec[two_power] + 1) % 3;

                self.rule = TotalisticSettingsScreen::rule_vec_to_code(rule_vec);
            }
        }

        None
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
            30.0, WHITE
        );

        for b in buttons.iter() {
            b.draw();
        }

        for i in 0..self.clickboxes.len() {
            self.draw_rule(&self.clickboxes[i], i as u8);
        }
    }
}

impl TotalisticSettingsScreen {
    pub fn draw_rule(&self, clickbox: &NumberClickbox, index: u8) {
        let space_offset: f32 = 10.0;
        let width: f32 = NumberClickbox::size() * 3.0 + (space_offset * 2.0);
        let height: f32 = NumberClickbox::size() * 3.0;

        let position = clickbox.position();

        let top_left = (
            position.0 - (width / 2.0),
            position.1 - height - NumberClickbox::size() / 2.0
        );

        clickbox.draw();

        if clickbox.mouse_over_box() {
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

            let opacity = (6.0 - index as f32) / 6.0;
            let rect_color = Color {
                a: opacity,
                ..WHITE
            };

            let rect_top_left = (
                top_left.0 + space_offset,
                top_left.1 + NumberClickbox::size() / 2.0
            );

            draw_rectangle(
                rect_top_left.0,
                rect_top_left.1,
                NumberClickbox::size() * 3.0,
                NumberClickbox::size(),
                rect_color
            );

            draw_rectangle_lines(
                rect_top_left.0,
                rect_top_left.1,
                NumberClickbox::size() * 3.0,
                NumberClickbox::size(),
                2.0,
                WHITE
            );

            let lower_color = match clickbox.state() {
                1 => GRAY,
                2 => WHITE,
                _ => BLACK,
            };
    
            draw_rectangle(
                clickbox.position().0 - NumberClickbox::size() / 2.0,
                top_left.1 + NumberClickbox::size() * 1.5,
                NumberClickbox::size(),
                NumberClickbox::size(),
                lower_color
            );
            draw_rectangle_lines(
                clickbox.position().0 - NumberClickbox::size() / 2.0,
                top_left.1 + NumberClickbox::size() * 1.5,
                NumberClickbox::size(),
                NumberClickbox::size(),
                2.0,
                WHITE,
            );
        }
    }

    fn rule_code_to_vec(rule: u16) -> Vec<u8> {
        let mut cur_rule = rule;
        let mut vec_rule: Vec<u8> = Vec::new();

        for _ in 0..7 {
            vec_rule.push((cur_rule % 3) as u8);
            cur_rule /= 3;
        }

        vec_rule
    }

    fn rule_vec_to_code(rule: Vec<u8>) -> u16 {
        let mut code_rule: u16 = 0;

        for (i, &v) in rule.iter().enumerate() {
            code_rule += v as u16 * 3_u16.pow(i as u32);
        }

        code_rule
    }

    fn random_rule(&mut self) {
        let mut rule: u16 = 0;

        for i in 0..self.clickboxes.len() {
            let exponent: u32 = 6 - i as u32;
            let value: u16 = rand::random_range(0..3);

            rule += value * 3_u16.pow(exponent) as u16;
            self.clickboxes[i].set(value as u32);
        }

        self.rule = rule
    }
}