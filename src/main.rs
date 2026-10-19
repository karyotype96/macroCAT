pub mod automata;
pub mod helpers;
pub mod toaster;
pub mod screens;

use macroquad::prelude::*;

use crate::{automata::{automaton_1d::{elementary::ElementaryConfig, extended::ExtendedConfig}, automaton_2d::lifelike::{LifelikeConfig, LifelikeNeighborhoodType}}, screens::{automaton_1d_screen::Automaton1DScreen, automaton_2d_screen::Automaton2DScreen, automaton_type_screen::AutomatonTypeScreen, elementary_settings::ElementarySettingsScreen, elementary_show::ElementaryShowScreen, extended_settings::ExtendedSettingsScreen, extended_show::ExtendedShowScreen, lifelike_settings::LifelikeSettingScreen, screen::Screen}, toaster::toaster::Toaster};

#[derive(Copy, Clone)]
pub enum MenuScreen {
    SelectAutomatonType,

    SelectAutomaton1D,
    SelectAutomaton2D,
    SelectAutomaton3D,

    ElementaryAutomatonOptions,
    ExtendedAutomatonOptions,

    LifelikeAutomatonOptions,
    
    ElementaryAutomatonShow(ElementaryConfig, u8),
    ExtendedAutomatonShow(ExtendedConfig, u32),

    LifelikeAutomatonShow(LifelikeConfig, LifelikeNeighborhoodType, (u16, u16))
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
    let mut automaton_2d_screen = Automaton2DScreen {};

    let mut elementary_settings_screen = ElementarySettingsScreen {
        checkboxes: Vec::new(),
        cfg: ElementaryConfig::CenterPixel,
        rule: 0
    };
    elementary_settings_screen.init();

    let mut extended_settings_screen = ExtendedSettingsScreen {
        checkboxes: Vec::new(),
        cfg: ExtendedConfig::CenterPixel,
        rule: 0
    };
    extended_settings_screen.init();

    let mut elementary_show_screen = ElementaryShowScreen::new();
    let mut extended_show_screen = ExtendedShowScreen::new();

    let mut lifelike_settings_screen = LifelikeSettingScreen {
        born_boxes: Vec::new(),
        live_boxes: Vec::new(),

        cfg: LifelikeConfig::None,
        neighborhood: LifelikeNeighborhoodType::Moore,
        rule: (0, 0)
    };
    lifelike_settings_screen.init();

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
                automaton_2d_screen.control(&mut current_scene, &mut toaster);
                automaton_2d_screen.draw();
            }
            MenuScreen::SelectAutomaton3D => {

            }

            MenuScreen::ElementaryAutomatonOptions => {
                elementary_settings_screen.control(&mut current_scene, &mut toaster);
                elementary_settings_screen.draw();
            }
            MenuScreen::ExtendedAutomatonOptions => {
                extended_settings_screen.control(&mut current_scene, &mut toaster);
                extended_settings_screen.draw();
            }

            MenuScreen::ElementaryAutomatonShow(cfg, rule) => {
                elementary_show_screen.set_info(cfg, rule);
                elementary_show_screen.control(&mut current_scene, &mut toaster);
                elementary_show_screen.draw();
            }
            MenuScreen::ExtendedAutomatonShow(cfg, rule) => {
                extended_show_screen.set_info(cfg, rule);
                extended_show_screen.control(&mut current_scene, &mut toaster);
                extended_show_screen.draw();
            }

            MenuScreen::LifelikeAutomatonOptions => {
                lifelike_settings_screen.control(&mut current_scene, &mut toaster);
                lifelike_settings_screen.draw();
            }

            MenuScreen::LifelikeAutomatonShow(cfg, neighborhood, (born, live)) => {

            }
        }

        toaster.run();

        next_frame().await
    }
}
