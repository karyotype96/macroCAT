pub mod automata;
pub mod helpers;
pub mod toaster;
pub mod screens;

use macroquad::prelude::*;

use crate::{automata::automaton_1d::{elementary::{ElementaryConfig}, extended::{ExtendedConfig}}, screens::{automaton_1d_screen::Automaton1DScreen, automaton_type_screen::AutomatonTypeScreen, elementary_settings::ElementarySettingsScreen, elementary_show::ElementaryShowScreen, screen::Screen}, toaster::toaster::Toaster};

#[derive(Copy, Clone)]
pub enum MenuScreen {
    SelectAutomatonType,

    SelectAutomaton1D,
    SelectAutomaton2D,
    SelectAutomaton3D,

    ElementaryAutomatonOptions,
    ExtendedAutomatonOptions,
    
    ElementaryAutomatonShow(ElementaryConfig, u8),
    ExtendedAutomatonShow(ExtendedConfig, u32),
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
    let mut current_scene = MenuScreen::SelectAutomatonType;

    let mut toaster = Toaster::new();

    let mut automaton_type_screen = AutomatonTypeScreen {};
    let mut automaton_1d_screen = Automaton1DScreen {};

    let mut elementary_settings_screen = ElementarySettingsScreen {
        checkboxes: Vec::new(),
        cfg: ElementaryConfig::CenterPixel,
        rule: 0
    };
    elementary_settings_screen.init();

    let mut elementary_show_screen = ElementaryShowScreen::new();
    
    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        match current_scene {
            MenuScreen::SelectAutomatonType => {
                automaton_type_screen.control(&mut current_scene, &mut toaster);
                automaton_type_screen.draw();
            }

            MenuScreen::SelectAutomaton1D => {
                automaton_1d_screen.control(&mut current_scene, &mut toaster);
                automaton_1d_screen.draw();
            }
            MenuScreen::SelectAutomaton2D => {
                
            }
            MenuScreen::SelectAutomaton3D => {

            }

            MenuScreen::ElementaryAutomatonOptions => {
                elementary_settings_screen.control(&mut current_scene, &mut toaster);
                elementary_settings_screen.draw();
            }
            MenuScreen::ExtendedAutomatonOptions => {

            }

            MenuScreen::ElementaryAutomatonShow(cfg, rule) => {
                elementary_show_screen.set_info(cfg, rule);
                elementary_show_screen.control(&mut current_scene, &mut toaster);
                elementary_show_screen.draw();
            }
            MenuScreen::ExtendedAutomatonShow(cfg, rule) => {

            }
        }

        toaster.run();

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
