use bevy::prelude::Color;

pub struct Config {
    pub enabled: bool,
    pub step_size: usize,
    pub x_len: usize,
    pub y_len: usize,
    pub rule: u8,
    pub probably_full: f32,
    pub full_color: Color,
    pub empty_color: Color,
    pub window_width: f32,
    pub window_height: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: false,
            step_size: 1,
            x_len: 300,
            y_len: 300,
            rule: 182,
            probably_full: 0.7,
            full_color: Color::WHITE,
            empty_color: Color::BLACK,
            window_width: 1000.0,
            window_height: 1000.0,
        }
    }
}
