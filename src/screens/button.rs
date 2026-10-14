use std::thread::current;

use macroquad::{color::{BLACK, Color, WHITE}, input::{MouseButton, is_mouse_button_pressed, mouse_position}, shapes::{draw_rectangle, draw_rectangle_lines}, text::{TextParams, draw_text_ex, measure_text}};

use crate::{MenuScreen, screens::screen::Screen};

const DEFAULT_FONT_SIZE: u16 = 24;
const DEFAULT_COLOR: Color = WHITE;

const OFFSET: f32 = 24.0;

pub struct Button {
    pub label: String,

    position: (f32, f32),
    dimensions: (f32, f32),
    color: Color,
    font_size: u16,
    callback: fn(&mut MenuScreen),
}

impl Button {
    pub fn new(label: &str, position: (f32, f32), font_size: Option<u16>, color: Option<Color>, callback: fn(&mut MenuScreen )) -> Self {
        let fsize = match font_size {
            Some(s) => s,
            None => DEFAULT_FONT_SIZE
        };
        let bcolor = match color {
            Some(c) => c,
            None => DEFAULT_COLOR
        };

        let text_dimensions = measure_text(
            label, 
            None, 
            fsize,
            1.0
        );

        let width = text_dimensions.width + OFFSET * 2.0;
        let height = text_dimensions.height + OFFSET * 2.0;
        
        Self {
            label: label.to_string(),
            position,
            dimensions: (width, height),
            color: bcolor,
            font_size: fsize,
            callback,
        }
    }

    pub fn get_dimensions(&self) -> (f32, f32) {
        self.dimensions
    }

    pub fn control(&self, current_screen: &mut MenuScreen) {
        let mouse_over_button = self.mouse_over_button();
        
        if is_mouse_button_pressed(MouseButton::Left) && mouse_over_button {
            (self.callback)(current_screen);
        }
    }

    pub fn draw(&self) {
        let (x, y) = self.position;
        let (width, height) = self.dimensions;
        let corner_x = x - (width / 2.0);
        let corner_y = y - (height / 2.0);

        let opacity = if self.mouse_over_button() { 1.0 } else { 0.7 };
        let color = Color {
            a: opacity,
            ..self.color
        };

        draw_rectangle(
            corner_x, corner_y,
            width, height,
            BLACK
        );

        draw_rectangle_lines(
            corner_x, corner_y, 
            width, height, 
            7.0,
            color
        );

        draw_text_ex(
            self.label.as_str(),
            corner_x + OFFSET,
            corner_y + height - OFFSET,
            TextParams {
                font: None,
                font_size: self.font_size,
                font_scale: 1.0, 
                font_scale_aspect: 1.0, 
                rotation: 0.0, 
                color,
            }
        );
    }

    fn mouse_over_button(&self) -> bool {
        let mouse_pos = mouse_position();

        let start_x = self.position.0 - (self.dimensions.0 / 2.0);
        let start_y = self.position.1 - (self.dimensions.1 / 2.0);

        mouse_pos.0 >= start_x &&
        mouse_pos.0 <= start_x + self.dimensions.0 &&
        mouse_pos.1 >= start_y &&
        mouse_pos.1 <= start_y + self.dimensions.1
    }
}