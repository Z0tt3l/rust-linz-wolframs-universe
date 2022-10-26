use bevy::prelude::Color;

use crate::config::Config;

#[derive(Clone)]
pub struct UIState {
    pub container_width: f32,
    pub container_height: f32,
    pub last_updated_age: usize,
    pub cell_size: f32,
    pub visualization_update_needed: bool,
    pub full_color: Color,
    pub empty_color: Color,
}

impl UIState {
    pub fn update_cell_size(&mut self, x_len: usize, container_width: f32) {
        self.cell_size = Self::get_new_cell_size(x_len, container_width);
    }

    pub fn get_new_cell_size(x_len: usize, container_width: f32) -> f32 {
        container_width / x_len as f32
    }
}

impl From<&Config> for UIState {
    fn from(config: &Config) -> Self {
        Self {
            container_height: config.window_height,
            container_width: config.window_width,
            cell_size: Self::get_new_cell_size(config.x_len, config.window_width),
            full_color: config.full_color,
            empty_color: config.empty_color,
            ..Default::default()
        }
    }
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            container_height: 500.0,
            container_width: 500.0,
            last_updated_age: 0,
            cell_size: 10.0,
            visualization_update_needed: false,
            full_color: Color::BLACK,
            empty_color: Color::WHITE,
        }
    }
}
