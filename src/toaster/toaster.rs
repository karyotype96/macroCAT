use std::{collections::VecDeque, time::Instant};

use macroquad::{color::{BLACK, Color, GREEN, RED, WHITE}, shapes::{draw_rectangle, draw_rectangle_lines}, text::{TextParams, draw_text_ex, measure_text}};

use crate::helpers::smoothing::EasingType;

pub const DEFAULT_APPEAR_TIME_MILLIS: u128 = 200;
pub const DEFAULT_SHOW_TIME_MILLIS: u128 = 2500;
pub const DEFAULT_FADE_TIME_MILLIS: u128 = 1000;

pub const FONT_SIZE: u16 = 24;

const OFFSET: f32 = 24.0;

pub enum MessageType {
    Success,
    Information,
    Error
}

pub struct ToastMessage {
    message: String,
    message_type: MessageType,
    appear_time: u128,
    show_time: u128,
    fade_time: u128,
    start_time: Instant,
}

impl ToastMessage {
    pub fn new(message: &str, message_type: Option<MessageType>, appear_time: Option<u128>, show_time: Option<u128>, fade_time: Option<u128>) -> Self {
        let msg_type = match message_type {
            Some(t) => t,
            None => MessageType::Information,
        };
        let appear = match appear_time {
            Some(s) => s,
            None => DEFAULT_APPEAR_TIME_MILLIS
        };
        let show = match show_time {
            Some(s) => s,
            None => DEFAULT_SHOW_TIME_MILLIS
        };
        let fade = match fade_time {
            Some(s) => s,
            None => DEFAULT_FADE_TIME_MILLIS
        };

        Self {
            message: String::from(message),
            message_type: msg_type,
            appear_time: appear,
            show_time: show,
            fade_time: fade,
            start_time: Instant::now()
        }
    }

    pub fn begin(&mut self) {
        self.start_time = Instant::now();
    }

    pub fn get_opacity(&self) -> f32 {
        let elapsed = self.start_time.elapsed().as_millis();
        if elapsed < self.appear_time + self.show_time {
            return 1.0;
        } else if elapsed > self.appear_time + self.show_time + self.fade_time {
            return 0.0;
        }

        let ease_type = EasingType::Linear;
        ease_type.ease_range((elapsed - (self.appear_time + self.show_time)) as f32, 0.0, self.fade_time as f32, 1.0, 0.0)
    }

    pub fn is_ended(&self) -> bool {
        return self.start_time.elapsed().as_millis() >= self.appear_time + self.show_time + self.fade_time;
    }

    pub fn draw(&self) {
        let text_dimensions = measure_text(self.message.as_str(), None, FONT_SIZE, 1.0);
        let color = match self.message_type {
            MessageType::Success => GREEN,
            MessageType::Information => WHITE,
            MessageType::Error => RED
        };

        let appear_ease_type = EasingType::OutSine;
        let time_elapsed = self.start_time.elapsed().as_millis();
        let x_position = appear_ease_type.ease_range(time_elapsed as f32, 0.0, self.appear_time as f32, -(text_dimensions.width - OFFSET * 1.2), OFFSET);

        draw_rectangle(
            x_position,
            OFFSET,
            text_dimensions.width + OFFSET * 2.0, 
            text_dimensions.height + OFFSET * 2.0,
            Color {
                a: self.get_opacity(),
                ..BLACK
            }
        );

        draw_rectangle_lines(
            x_position, 
            OFFSET, 
            text_dimensions.width + OFFSET * 2.0, 
            text_dimensions.height + OFFSET * 2.0, 
            3.0, 
            Color {
                a: self.get_opacity(),
                ..color
            }
        );

        draw_text_ex(
            self.message.as_str(),
            x_position + OFFSET,
            text_dimensions.height + OFFSET * 1.7,
            TextParams { 
                font: None, 
                font_size: FONT_SIZE,
                font_scale: 1.0, 
                font_scale_aspect: 1.0, 
                rotation: 0.0, 
                color: Color {
                    a: self.get_opacity(),
                    ..color
                }
            }
        );
    }
}

pub struct Toaster {
    toasts: VecDeque<ToastMessage>,
}

impl Toaster {
    pub fn new() -> Self {
        Self {
            toasts: VecDeque::new(),
        }
    }

    pub fn add_message(&mut self, 
        msg: &str, 
        msg_type: Option<MessageType>, 
        appear_time: Option<u128>, 
        show_time: Option<u128>, 
        fade_time: Option<u128>
    ) {
        self.toasts.push_back(
            ToastMessage::new(msg, msg_type, appear_time, show_time, fade_time)
        );
    }

    pub fn quick_msg(&mut self, msg: &str) {
        self.add_message(msg,
            None, None, None, None
        );
    }

    pub fn flush(&mut self) {
        self.toasts.clear();
    }

    pub fn run(&mut self) {
        if self.toasts.len() > 0 {
            self.toasts[0].draw();

            if self.toasts[0].is_ended() {
                self.toasts.pop_front();
                if self.toasts.len() > 0 {
                    self.toasts[0].begin();
                }
            }
        }
    }
}