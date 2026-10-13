use macroquad::{color::{BLACK, Color, WHITE}, input::{MouseButton, is_mouse_button_pressed, mouse_position}, shapes::{draw_rectangle, draw_rectangle_lines}, text::{TextParams, draw_text_ex, measure_text}};

const DEFAULT_FONT_SIZE: u16 = 24;
const DEFAULT_COLOR: Color = WHITE;

const OFFSET: f32 = 24.0;

pub struct Button {
    pub label: String,

    position: (f32, f32),
    dimensions: (f32, f32),
    color: Color,
    font_size: u16,
    callback: Option<fn()>,
}

impl Button {
    pub fn new(label: &str, position: (f32, f32), font_size: Option<u16>, color: Option<Color>, callback: Option<fn()>) -> Self {
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

    pub fn attach_callback(&mut self, callback: Option<fn()>) {
        self.callback = callback;
    }

    pub fn control(&self) {
        let (x, y) = self.position;

        let mouse_pos = mouse_position();

        let mouse_over_button = 
            mouse_pos.0 >= x &&
            mouse_pos.0 <= x + self.dimensions.0 &&
            mouse_pos.1 >= y &&
            mouse_pos.1 <= y + self.dimensions.1;
        
        if is_mouse_button_pressed(MouseButton::Left) && mouse_over_button {
            if let Some(callback) = self.callback {
                callback();
            }
        }
    }

    pub fn draw(&self) {
        let (x, y) = self.position;
        let (width, height) = self.dimensions;
        let opacity = if self.mouse_over_button() { 1.0 } else { 0.7 };
        let color = Color {
            a: opacity,
            ..self.color
        };

        draw_rectangle(
            x, y,
            width, height,
            BLACK
        );

        draw_rectangle_lines(
            x, y, 
            width, height, 
            7.0,
            color
        );

        draw_text_ex(
            self.label.as_str(),
            x + OFFSET,
            y + height - OFFSET,
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

        mouse_pos.0 >= self.position.0 &&
        mouse_pos.0 <= self.position.0 + self.dimensions.0 &&
        mouse_pos.1 >= self.position.1 &&
        mouse_pos.1 <= self.position.1 + self.dimensions.1
    }
}