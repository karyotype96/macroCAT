use macroquad::{color::{GRAY, GREEN, WHITE}, input::{MouseButton, is_mouse_button_pressed, mouse_position}, shapes::{draw_rectangle, draw_rectangle_lines}, text::draw_text};

const CHECKBOX_SIZE: f32 = 30.0;

pub struct Checkbox {
    label: Option<String>,
    position: (f32, f32),
    is_checked: bool,
}

impl Checkbox {
    pub fn new(label: Option<String>, position: (f32, f32)) -> Self {
        Self {
            label,
            position,
            is_checked: false,
        }
    }

    pub fn checked(&self) -> bool {
        self.is_checked
    }

    pub fn mouse_over_box(&self) -> bool {
        let mouse_pos = mouse_position();

        let start_x = self.position.0 - (CHECKBOX_SIZE / 2.0);
        let start_y = self.position.1 - (CHECKBOX_SIZE / 2.0);

        mouse_pos.0 >= start_x &&
        mouse_pos.0 <= start_x + CHECKBOX_SIZE &&
        mouse_pos.1 >= start_y &&
        mouse_pos.1 <= start_y + CHECKBOX_SIZE
    }

    pub fn control(&mut self) -> bool {
        let mouse_over_box = self.mouse_over_box();

        if is_mouse_button_pressed(MouseButton::Left) && mouse_over_box {
            self.is_checked = !self.is_checked;
            return true;
        }

        false
    }

    pub fn size() -> f32 {
        CHECKBOX_SIZE
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.position.0 - (CHECKBOX_SIZE / 2.0), 
            self.position.1 - (CHECKBOX_SIZE / 2.0), 
            CHECKBOX_SIZE, CHECKBOX_SIZE, 
            WHITE
        );

        draw_rectangle_lines(
            self.position.0 - (CHECKBOX_SIZE / 2.0),
            self.position.1 - (CHECKBOX_SIZE / 2.0),
            CHECKBOX_SIZE, CHECKBOX_SIZE, 
            3.0, 
            if self.mouse_over_box() { GREEN } else { GRAY },
        );

        if self.is_checked {
            println!("is checked");
            draw_rectangle(
                self.position.0 - (CHECKBOX_SIZE / 2.0) + 5.0,
                self.position.1 - (CHECKBOX_SIZE / 2.0) + 5.0,
                CHECKBOX_SIZE - 10.0, CHECKBOX_SIZE - 10.0, 
                GRAY
            );
        }

        if let Some(l) = &self.label {
            draw_text(
                l.clone(),
                self.position.0 + (CHECKBOX_SIZE / 2.0) + 5.0,
                self.position.1 + (CHECKBOX_SIZE / 2.0),
                24.0,
                WHITE,
            );
        }
    }
}