use macroquad::input::{KeyCode, is_key_pressed};
use petgraph::{Graph, graph::NodeIndex};

use crate::{screens::{automata_1d::{automaton_1d_screen::Automaton1DScreen, elementary_settings::ElementarySettingsScreen, elementary_show::ElementaryShowScreen, extended_settings::ExtendedSettingsScreen, extended_show::ExtendedShowScreen, totalistic_settings::TotalisticSettingsScreen, totalistic_show::TotalisticShowScreen}, automata_2d::{automaton_2d_screen::Automaton2DScreen, brians_brain_settings::BriansBrainSettingsScreen, brians_brain_show::BriansBrainShowScreen, lifelike_settings::LifelikeSettingsScreen, lifelike_show::LifelikeShowScreen}, automaton_type_screen::AutomatonTypeScreen, screen::Screen}, toaster::toaster::Toaster};

#[derive(Default)]
pub enum MenuScreen {
    #[default]
    None,
    // starting screen
    SelectAutomatonType(AutomatonTypeScreen),

    SelectAutomaton1D(Automaton1DScreen),
    SelectAutomaton2D(Automaton2DScreen),

    // 1D automata options
    ElementaryAutomatonOptions(ElementarySettingsScreen),
    ExtendedAutomatonOptions(ExtendedSettingsScreen),
    TotalisticAutomatonOptions(TotalisticSettingsScreen),

    // 2D automata
    LifelikeAutomatonOptions(LifelikeSettingsScreen),
    BriansBrainOptions(BriansBrainSettingsScreen),
    
    // 1D automata show
    ElementaryAutomatonShow(ElementaryShowScreen),
    ExtendedAutomatonShow(ExtendedShowScreen),
    TotalisticAutomatonShow(TotalisticShowScreen),

    // 2D automata show
    LifelikeAutomatonShow(LifelikeShowScreen),
    BriansBrainShow(BriansBrainShowScreen)
}

pub struct Nav {
    tree: Graph<MenuScreen, MenuScreen>,
    indices: Vec<NodeIndex>,

    pub toaster: Toaster
}

impl Nav {
    pub fn new() -> Self {
        // Note: all navigational data is to be declared here instead of
        // in main.rs from now on

        let mut tree = Graph::new();

        let sat = tree.add_node(MenuScreen::SelectAutomatonType(
            AutomatonTypeScreen{}
        ));
        
        let s1da = tree.add_node(MenuScreen::SelectAutomaton1D(
            Automaton1DScreen{}
        ));
        let s2da = tree.add_node(MenuScreen::SelectAutomaton2D(
            Automaton2DScreen{}
        ));

        let elao = tree.add_node(MenuScreen::ElementaryAutomatonOptions(
            ElementarySettingsScreen::new()
        ));
        let exao = tree.add_node(MenuScreen::ExtendedAutomatonOptions(
            ExtendedSettingsScreen::new()
        ));
        let tao = tree.add_node(MenuScreen::TotalisticAutomatonOptions(
            TotalisticSettingsScreen::new()
        ));

        let lao = tree.add_node(MenuScreen::LifelikeAutomatonOptions(
            LifelikeSettingsScreen::new()
        ));
        let bbo = tree.add_node(MenuScreen::BriansBrainOptions(
            BriansBrainSettingsScreen::new()
        ));

        let els = tree.add_node(
            MenuScreen::ElementaryAutomatonShow(
                ElementaryShowScreen::new()
            )
        );
        let exs = tree.add_node(
            MenuScreen::ExtendedAutomatonShow(
                ExtendedShowScreen::new()
            )
        );
        let ts = tree.add_node(
            MenuScreen::TotalisticAutomatonShow(
                TotalisticShowScreen::new()
            )
        );

        let las = tree.add_node(
            MenuScreen::LifelikeAutomatonShow(
                LifelikeShowScreen::new()
            )
        );
        let bbs = tree.add_node(
            MenuScreen::BriansBrainShow(
                BriansBrainShowScreen::new()
            )
        );

        tree.extend_with_edges(&[
            (sat, s2da),
            (sat, s1da),

            (s1da, tao),
            (s1da, exao),
            (s1da, elao),
            
            (elao, els),

            (exao, exs),

            (tao, ts),

            (s2da, bbo),
            (s2da, lao),

            (lao, las),

            (bbo, bbs)
        ]);

        Self {
            tree,
            indices: vec![sat],
            toaster: Toaster::new(),
        }
    }

    pub fn get_current_node(&mut self) -> NodeIndex {
        let ind = self.indices.last();

        let x = match ind {
            Some(n) => *n,
            None => NodeIndex::new(0)
        };

        x
    }

    pub fn get_current_screen(&mut self) -> &mut MenuScreen {
        let node = self.get_current_node();

        &mut self.tree[node]
    }

    pub fn navigate_forward(&mut self, nav_index: usize) -> NodeIndex {
        let current_index = self.get_current_node();

        let neighbors = self.tree.neighbors(current_index);

        let next: Vec<NodeIndex> = neighbors
            .enumerate()
            .filter(|(i, _)| { 
                *i == nav_index
            })
            .map(|(_, value)| { value })
            .collect();

        self.indices.push(next[0]);
        next[0]
    }
    
    pub fn navigate_back(&mut self) {
        if self.indices.len() > 1 {
            self.indices.pop();
        }
    }

    pub fn control(&mut self) {
        if is_key_pressed(KeyCode::Q) {
            self.navigate_back();
            return;
        }

        let screen = self.get_current_screen();

        match screen {
            MenuScreen::SelectAutomatonType(s) => {
                if let Some(i) = s.control() {
                    self.navigate_forward(i);
                }
            }
            MenuScreen::SelectAutomaton1D(s) => {
                if let Some(i) = s.control() {
                    self.navigate_forward(i);
                }
            }
            MenuScreen::SelectAutomaton2D(s) => {
                if let Some(i) = s.control() {
                    self.navigate_forward(i);
                }
            }
            MenuScreen::ElementaryAutomatonOptions(s) => {
                let info = s.get_info();
                
                if let Some(i) = s.control() {
                    let next_screen = self.navigate_forward(i);

                    let to_edit = &mut self.tree[next_screen];
                    match to_edit {
                        MenuScreen::ElementaryAutomatonShow(ns) => {
                            ns.reset();
                            ns.set_info(info.0, info.1);
                        }
                        _ => {}
                    }
                }
            }
            MenuScreen::ExtendedAutomatonOptions(s) => {
                let info = s.get_info();
                
                if let Some(i) = s.control() {
                    let next_screen = self.navigate_forward(i);

                    let to_edit = &mut self.tree[next_screen];
                    match to_edit {
                        MenuScreen::ExtendedAutomatonShow(ns) => {
                            ns.reset();
                            ns.set_info(info.0, info.1);
                        }
                        _ => {}
                    }
                }
            }
            MenuScreen::TotalisticAutomatonOptions(s) => {
                let info = s.get_info();
                
                if let Some(i) = s.control() {
                    let next_screen = self.navigate_forward(i);
                
                    let to_edit = &mut self.tree[next_screen];
                    match to_edit {
                        MenuScreen::TotalisticAutomatonShow(ns) => {
                            ns.reset();
                            ns.set_info(info.0, info.1);
                        }
                        _ => {}
                    }
                }
            }
            MenuScreen::LifelikeAutomatonOptions(s) => {
                let info = s.get_info();

                if let Some(i) = s.control() {
                    let next_screen = self.navigate_forward(i);
                    
                    let to_edit = &mut self.tree[next_screen];
                    match to_edit {
                        MenuScreen::LifelikeAutomatonShow(ns) => { 
                            ns.reset();
                            ns.set_info(info.0, info.1, info.2);
                        }
                        _ => {}
                    }
                }
            }
            MenuScreen::BriansBrainOptions(s) => {
                let info = s.get_info();

                if let Some(i) = s.control() {
                    let next_screen = self.navigate_forward(i);
                    
                    let to_edit = &mut self.tree[next_screen];
                    match to_edit {
                        MenuScreen::BriansBrainShow(ns) => { 
                            ns.reset();
                            ns.set_info(info);
                        }
                        _ => {}
                    }
                }
            }
            MenuScreen::ElementaryAutomatonShow(s) => {
                s.control();
            }
            MenuScreen::ExtendedAutomatonShow(s) => {
                s.control();
            }
            MenuScreen::TotalisticAutomatonShow(s) => {
                s.control();
            }
            MenuScreen::LifelikeAutomatonShow(s) => {
                s.control();
            }
            MenuScreen::BriansBrainShow(s) => {
                s.control();
            }
            _ => {}
        }
    }

    pub fn draw(&mut self) {
        let screen = self.get_current_screen();

        match screen {
            MenuScreen::SelectAutomatonType(s) => { s.draw(); }
            MenuScreen::SelectAutomaton1D(s) => { s.draw(); }
            MenuScreen::SelectAutomaton2D(s) => { s.draw(); }
            MenuScreen::ElementaryAutomatonOptions(s) => { s.draw(); }
            MenuScreen::ExtendedAutomatonOptions(s) => { s.draw(); }
            MenuScreen::TotalisticAutomatonOptions(s) => { s.draw(); }
            MenuScreen::LifelikeAutomatonOptions(s) => { s.draw(); }
            MenuScreen::BriansBrainOptions(s) => { s.draw(); }
            MenuScreen::ElementaryAutomatonShow(s) => { s.draw(); }
            MenuScreen::ExtendedAutomatonShow(s) => { s.draw(); }
            MenuScreen::TotalisticAutomatonShow(s) => { s.draw(); }
            MenuScreen::LifelikeAutomatonShow(s) => { s.draw(); }
            MenuScreen::BriansBrainShow(s) => { s.draw(); }
            _ => {}
        }
    }
}