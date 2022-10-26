use bevy::{
    math::Vec2,
    prelude::{Color, Commands, DespawnRecursiveExt, Entity, Query, SpriteBundle, Transform},
    sprite::Sprite,
};

mod cell;
mod control_state;
mod ui_state;

pub use cell::Cell;
pub use control_state::ControlState;
pub use ui_state::UIState;

pub fn despawn_entities(commands: &mut Commands, cell_query: &Query<(Entity, &Cell)>) {
    for (entity, _) in cell_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn get_cell_sprite_bundle(
    color: Color,
    x_position: f32,
    y_position: f32,
    cell_size: f32,
) -> SpriteBundle {
    SpriteBundle {
        transform: Transform::from_xyz(x_position, y_position, 0.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(cell_size, cell_size)),
            color,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn get_x_position(index: usize, cell_size: f32, width: f32) -> f32 {
    (cell_size * index as f32) - (width / 2.0) + (cell_size / 2.0)
}

pub fn get_y_position(index: usize, cell_size: f32, height: f32) -> f32 {
    (height / 2.0) - (cell_size * index as f32) - (cell_size / 2.0)
}
