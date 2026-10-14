pub mod automata;
pub mod helpers;
pub mod toaster;
pub mod screens;

use std::thread::current;

use macroquad::prelude::*;
use ::rand::random;

use crate::{automata::automaton_1d::{base::Automaton1D, elementary::{Elementary, ElementaryConfig}, extended::{Extended, ExtendedConfig}}, screens::{screen::Screen, automaton_type_screen::AutomatonTypeScreen, button::Button}, toaster::toaster::Toaster};

#[derive(Copy, Clone)]
pub enum MenuScreen {
    SelectAutomatonType,

    SelectAutomaton1D,
    SelectAutomaton2D,
    SelectAutomaton3D,

    ElementaryAutomatonOptions,
    ExtendedAutomatonOptions,
    
    ElementaryAutomatonShow,
    ExtendedAutomatonShow,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "MacroCAT".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    let cfg = ExtendedConfig::CenterPixel;

    let mut current_scene = MenuScreen::SelectAutomatonType;

    let mut rule: u32 = random();

    let mut el = Extended::new(2000, cfg, rule);
    let mut toaster = Toaster::new();

    let automaton_type_screen = AutomatonTypeScreen {};

    toaster.add_message(
        format!("Rule selected: {}", rule).as_str(),
        None,
        None,
        Some(10000),
        None
    );
    
    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        match current_scene {
            MenuScreen::SelectAutomatonType => {
                automaton_type_screen.control(&mut current_scene);
                automaton_type_screen.draw();
            }

            MenuScreen::SelectAutomaton1D => {

            }
            MenuScreen::SelectAutomaton2D => {

            }
            MenuScreen::SelectAutomaton3D => {

            }

            MenuScreen::ElementaryAutomatonOptions => {

            }
            MenuScreen::ExtendedAutomatonOptions => {

            }

            MenuScreen::ElementaryAutomatonShow => {

            }
            MenuScreen::ExtendedAutomatonShow => {

            }
        }

        /*
        if is_key_pressed(KeyCode::Backspace) {
            toaster.flush();
            rule = random();
            toaster.add_message(
                format!("Rule selected: {}", rule).as_str(),
                None,
                None,
                Some(10000),
                None
            );

            el = Extended::new(2000, cfg, rule);
        }
        if is_key_pressed(KeyCode::M) {
            toaster.flush();
            toaster.add_message(
                format!("Rule selected: {}", rule).as_str(),
                None,
                None,
                Some(10000),
                None
            );
        }

        el.control();
        el.draw();
        toaster.run();
        */

        next_frame().await
    }
}
