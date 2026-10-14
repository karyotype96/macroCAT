use crate::{MenuScreen, screens::button::Button};

pub trait Screen {
    fn get_buttons(&self) -> Vec<Button>;
    fn control(&self, current_screen: &mut MenuScreen);
    fn draw(&self);
}