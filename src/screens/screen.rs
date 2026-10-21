use crate::{MenuScreen, screens::{button::MenuButton}, toaster::toaster::Toaster};

pub trait Screen {
    fn init(&mut self);
    fn get_buttons(&self) -> Vec<MenuButton>;
    fn control(&mut self, current_screen: &mut MenuScreen, toaster: &mut Toaster);
    fn draw(&self);
}