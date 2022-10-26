use bevy::prelude::Component;

#[derive(Component, Debug)]
pub struct Cell {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub state: bool,
}
