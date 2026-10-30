use crate::{screens::{button::MenuButton}};

pub trait Screen<T> {
    fn new() -> Self;
    fn init(&mut self);
    fn get_buttons(&self) -> Vec<MenuButton>;
    fn get_info(&self) -> T;
    fn control(&mut self) -> Option<usize>;
    fn draw(&self);
}