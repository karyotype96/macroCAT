use crate::{MenuScreen, screens::{button::Button, checkbox::Checkbox}, toaster::toaster::Toaster};

pub trait Screen {
    fn init(&mut self);
    fn get_buttons(&self) -> Vec<Button>;
    fn control(&mut self, current_screen: &mut MenuScreen);
    fn draw(&self);
}