use macroquad::{color::{BLACK, Color, GRAY, GREEN, WHITE}, input::{MouseButton, is_mouse_button_pressed, mouse_position}, shapes::{draw_rectangle, draw_rectangle_lines}, text::{draw_text, measure_text}};

const CLICKBOX_SIZE: f32 = 30.0;

pub struct NumberClickbox {
    label: Option<String>,
    position: (f32, f32),
    state: u32,
    max_state: u32,
}

impl NumberClickbox {
    pub fn new(label: Option<String>, position: (f32, f32), max_state: u32) -> Self {
        Self {
            label,
            position,
            state: 0,
            max_state
        }
    }

    pub fn set(&mut self, state: u32) {
        self.state = state % self.max_state;
    }

    pub fn state(&self) -> u32 {
        self.state
    }

    pub fn mouse_over_box(&self) -> bool {
        let mouse_pos = mouse_position();

        let start_x = self.position.0 - (CLICKBOX_SIZE / 2.0);
        let start_y = self.position.1 - (CLICKBOX_SIZE / 2.0);

        mouse_pos.0 >= start_x &&
        mouse_pos.0 <= start_x + CLICKBOX_SIZE &&
        mouse_pos.1 >= start_y &&
        mouse_pos.1 <= start_y + CLICKBOX_SIZE
    }

    pub fn control(&mut self) -> bool {
        let mouse_over_box = self.mouse_over_box();

        if is_mouse_button_pressed(MouseButton::Left) && mouse_over_box {
            self.state = (self.state + 1) % self.max_state;
            return true;
        }

        false
    }

    pub fn size() -> f32 {
        CLICKBOX_SIZE
    }

    pub fn position(&self) -> (f32, f32) {
        self.position
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.position.0 - (CLICKBOX_SIZE / 2.0),
            self.position.1 - (CLICKBOX_SIZE / 2.0),
            CLICKBOX_SIZE, CLICKBOX_SIZE,
            WHITE
        );

        draw_rectangle_lines(
            self.position.0 - (CLICKBOX_SIZE / 2.0),
            self.position.1 - (CLICKBOX_SIZE / 2.0),
            CLICKBOX_SIZE, CLICKBOX_SIZE, 
            3.0,
            if self.mouse_over_box() { GREEN } else { GRAY }
        );

        if self.state > 0 {
            let color = Color {
                a: (self.state as f32) / (self.max_state as f32),
                ..BLACK
            };

            draw_rectangle(
                self.position.0 - (CLICKBOX_SIZE / 2.0) + 5.0,
                self.position.1 - (CLICKBOX_SIZE / 2.0) + 5.0,
                CLICKBOX_SIZE - 10.0, CLICKBOX_SIZE - 10.0,
                color
            );
        }

        if let Some(l) = &self.label {
            draw_text(l.clone(),
                self.position.0 + (CLICKBOX_SIZE / 2.0) + 5.0, 
                self.position.1 + (CLICKBOX_SIZE / 2.0), 
                24.0, 
                WHITE
            );
        }

        let number_label = format!("{}", self.state);
        let dimensions = measure_text(
            number_label.as_str(),
            None,
            16,
            1.0
        );

        let number_label_x = self.position.0 - dimensions.width / 2.0;

        draw_text(
            number_label,
            number_label_x,
            self.position.1 + (CLICKBOX_SIZE / 2.0) + 16.0,
            16.0,
            WHITE
        );
    }
}