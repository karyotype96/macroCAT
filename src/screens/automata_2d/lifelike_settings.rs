use macroquad::{color::{BLUE, WHITE}, text::{draw_text, measure_text}, window::{screen_height, screen_width}};
use rand::random_range;

use crate::{automata::automaton_2d::lifelike::{LifelikeConfig, LifelikeNeighborhoodType}, screens::{button::MenuButton, checkbox::Checkbox, screen::Screen}};

pub struct LifelikeSettingsScreen {
    pub born_boxes: Vec<Checkbox>, // determines the rule for when a cell is born
    pub live_boxes: Vec<Checkbox>, // determines the rule for when a cell remains alive

    pub cfg: LifelikeConfig,
    pub neighborhood: LifelikeNeighborhoodType,
    pub rule: (u16, u16)
}

impl Screen<(LifelikeConfig, LifelikeNeighborhoodType, (u16, u16))> for LifelikeSettingsScreen {
    fn new() -> Self {
        let mut screen = LifelikeSettingsScreen {
            born_boxes: Vec::new(),
            live_boxes: Vec::new(),

            cfg: LifelikeConfig::None,
            neighborhood: LifelikeNeighborhoodType::Moore,
            rule: (0, 0)
        };

        screen.init();
        screen
    }
    
    fn init(&mut self) {
        let left_center = screen_width() / 3.0;
        let right_center = screen_width() * 2.0 / 3.0;

        let leftmost_x_left = left_center - Checkbox::size()*4.0;
        let leftmost_x_right = right_center - Checkbox::size()*4.0;

        for i in 0..9 {
            self.born_boxes.push(Checkbox::new(
                None,
                (leftmost_x_left + Checkbox::size()*(i as f32), screen_height() / 3.0)
            ));
        }

        for i in 0..9 {
            self.live_boxes.push(Checkbox::new(
                None,
                (leftmost_x_right + Checkbox::size()*(i as f32), screen_height() / 3.0)
            ));
        }
    }

    fn get_buttons(&self) -> Vec<MenuButton> {
        let config_button_label = match self.cfg {
            LifelikeConfig::None => String::from("CONFIG: NONE"),
            LifelikeConfig::AllRandom => String::from("CONFIG: ALL RANDOM"),
            LifelikeConfig::RandomCenter(n) => format!("CONFIG: RANDOM CENTER {v}x{v}", v=n),
        };

        let config_button = MenuButton::new(
            config_button_label.as_str(),
            (screen_width() / 2.0 - 200.0, screen_height() * 2.0 / 3.0),
            None,
            None
        );

        let neighborhood_button_label = match self.neighborhood {
            LifelikeNeighborhoodType::Moore => String::from("NEIGHBORHOOD: MOORE"),
            LifelikeNeighborhoodType::VonNeumann => String::from("NEIGHBORHOOD: VON NEUMANN"),
        };

        let neighborhood_button = MenuButton::new(
            neighborhood_button_label.as_str(),
            (screen_width() / 2.0 + 200.0, screen_height() * 2.0 / 3.0),
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
            "SELECT RANDOM RULE",
            (screen_width() / 2.0, screen_height() / 2.0),
            None,
            None
        );

        vec![config_button, neighborhood_button, start_button, random_rule_button]
    }

    fn get_info(&self) -> (LifelikeConfig, LifelikeNeighborhoodType, (u16, u16)) {
        (self.cfg, self.neighborhood, self.rule)
    }

    fn control(&mut self) -> Option<usize> {
        let buttons = self.get_buttons();

        if buttons[0].control() {
            self.cfg = match self.cfg {
                LifelikeConfig::None => LifelikeConfig::AllRandom,
                LifelikeConfig::AllRandom => LifelikeConfig::RandomCenter(5),
                LifelikeConfig::RandomCenter(_) => LifelikeConfig::None
            };
        }

        if buttons[1].control() {
            self.neighborhood = match self.neighborhood {
                LifelikeNeighborhoodType::Moore => LifelikeNeighborhoodType::VonNeumann,
                LifelikeNeighborhoodType::VonNeumann => LifelikeNeighborhoodType::Moore
            }
        }

        if buttons[2].control() {
            return Some(0);
        }

        if buttons[3].control() {
            self.random_rule();
        }

        for i in 0..self.born_boxes.len() {
            if self.born_boxes[i].control() {
                let bit: u16 = 8 - i as u16;
                self.flip_born_bit(bit, self.born_boxes[i].checked());
            }
        }

        for i in 0..self.live_boxes.len() {
            if self.live_boxes[i].control(){
                let bit: u16 = 8 - i as u16;
                self.flip_live_bit(bit, self.live_boxes[i].checked());
            }
        }

        None
    }

    fn draw(&self) {
        let buttons = self.get_buttons();

        let rule = self.rule_to_string();
        let label = match rule.as_str() {
            "B0123478/S34678" => "InverseLife",
            "B1/S1" => "Gnarl",
            "B1357/S1357" => "Replicator",
            "B2/S" => "Seeds",
            "B234/S" => "Serviettes",
            "B3/S012345678" => "Life Without Death",
            "B3/S1234" => "Mazectric",
            "B3/S12345" => "Maze",
            "B3/S23" => "Conway's Game of Life",
            "B3/S45678" => "Coral",
            "B34/S34" => "34 Life",
            "B345/S4567" => "Assimiliation",
            "B35678/S5678" => "Diamoeba",
            "B357/S1358" => "Amoeba",
            "B357/S238" => "Pseudo Life",
            "B36/S125" => "2x2",
            "B36/S23" => "HighLife",
            "B3678/S235678" => "Stains",
            "B3678/S34678" => "Day & Night",
            "B368/S245" => "Morley",
            "B378/S235678" => "Coagulations",
            "B45678/S2345" => "WalledCities",
            "B4678/S35678" => "Anneal",
            _ => "",
        };

        let label = if label.len() > 0 { 
            format!("Rule Selected: {} ({})", self.rule_to_string(), label)
        } else {
            format!("Rule Selected: {}", self.rule_to_string())
        };

        let label_dimensions = measure_text(
            label.clone(),
            None, 
            30,
            1.0
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
        
        for i in 0..self.born_boxes.len() {
            self.born_boxes[i].draw();
            draw_text(
                format!("{}", 8-i).as_str(), 
                self.born_boxes[i].position().0-5.0, 
                self.born_boxes[i].position().1 + 30.0, 
                20.0, WHITE
            );

            self.live_boxes[i].draw();
            draw_text(
                format!("{}", 8-i).as_str(), 
                self.live_boxes[i].position().0-5.0, 
                self.live_boxes[i].position().1 + 30.0, 
                20.0, WHITE
            );
        }
    }
}

impl LifelikeSettingsScreen {
    fn rule_to_string(&self) -> String {
        let mut born_rule = String::new();
        let mut live_rule = String::new();

        for i in 0..9 {
            if self.rule.0 & 1 << i > 0 {
                born_rule.push_str(&i.to_string());
            }
            if self.rule.1 & 1 << i > 0 {
                live_rule.push_str(&i.to_string());
            }
        }

        format!("B{}/S{}", born_rule, live_rule)
    }

    fn random_rule(&mut self) {
        let born_rule: u16 = random_range(0..512);
        let live_rule: u16 = random_range(0..512);

        self.rule = (born_rule, live_rule);
        for i in 0..self.born_boxes.len() {
            let bit_index = 8-i;

            if self.rule.0 & 1 << bit_index > 0 {
                self.born_boxes[i].set(true);
            } else {
                self.born_boxes[i].set(false);
            }

            if self.rule.1 & 1 << bit_index as u16 > 0 {
                self.live_boxes[i].set(true);
            } else {
                self.live_boxes[i].set(false);
            }
        }
    }

    fn flip_born_bit(&mut self, bit: u16, is_one: bool) {
        if bit >= 9 { return; }

        if is_one {
            self.rule.0 |= 1 << bit;
        } else {
            self.rule.0 &= !(1 << bit);
        }
    }

    fn flip_live_bit(&mut self, bit: u16, is_one: bool) {
        if bit >= 9 { return; }

        if is_one {
            self.rule.1 |= 1 << bit;
        } else {
            self.rule.1 &= !(1 << bit);
        }
    }
}