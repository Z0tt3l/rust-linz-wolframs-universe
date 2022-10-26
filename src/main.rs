use automata::wolframs_universe::WolframsUniverse;
use bevy::ecs::event::Events;
use bevy::input::{keyboard::KeyCode, Input};
use bevy::prelude::*;
use bevy::window::WindowResized;
use config::Config;
use ui::*;

mod automata;
mod config;
mod ui;

fn main() {
    let config = Config::default();
    let universe = WolframsUniverse::from(&config);
    let control_state = ControlState::from(&config);
    let ui_state = UIState::from(&config);

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: String::from("Wolfram's Universe"),
            width: config.window_width,
            height: config.window_height,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(config)
        .insert_resource(ui_state)
        .insert_resource(control_state)
        .insert_resource(universe)
        .add_startup_system(setup.label("setup"))
        .add_system(resize_notificator.label("resize_notificator"))
        .add_system(keyboard_input_system.label("keyboard_input_system"))
        .add_system(
            iterate_universe
                .after("keyboard_input_system")
                .label("iterate_universe"),
        )
        .add_system(update_visualization.after("iterate_universe"))
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn keyboard_input_system(
    mut commands: Commands,
    config: Res<Config>,
    keyboard_input: Res<Input<KeyCode>>,
    mut control_state: ResMut<ControlState>,
    mut ui_state: ResMut<UIState>,
    cell_query: Query<(Entity, &Cell)>,
    mut universe: ResMut<WolframsUniverse>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        control_state.enabled = !control_state.enabled;
    }

    if keyboard_input.just_pressed(KeyCode::S) {
        for _ in 0..control_state.step_size {
            universe.iterate();
        }
        ui_state.visualization_update_needed = true;
    }

    if keyboard_input.just_pressed(KeyCode::R) {
        despawn_entities(&mut commands, &cell_query);

        universe.recreate(config.x_len, config.probably_full, config.rule);
        ui_state.visualization_update_needed = true;
    }

    if keyboard_input.just_pressed(KeyCode::M) {
        despawn_entities(&mut commands, &cell_query);

        universe.recreate_with_full_middle(config.x_len, config.rule);
        ui_state.visualization_update_needed = true;
    }
}

fn resize_notificator(
    mut commands: Commands,
    mut ui_state: ResMut<UIState>,
    universe_state: Res<WolframsUniverse>,
    cell_query: Query<(Entity, &Cell)>,
    resize_event: Res<Events<WindowResized>>,
) {
    let mut reader = resize_event.get_reader();

    for event in reader.iter(&resize_event) {
        ui_state.container_width = event.width;
        ui_state.container_height = event.height;

        despawn_entities(&mut commands, &cell_query);

        ui_state.update_cell_size(universe_state.x_len, event.width);
        ui_state.last_updated_age = 0;
        ui_state.visualization_update_needed = true;
    }
}

fn iterate_universe(
    control_state: Res<ControlState>,
    mut ui_state: ResMut<UIState>,
    mut universe: ResMut<WolframsUniverse>,
) {
    if control_state.enabled {
        for _ in 0..control_state.step_size {
            universe.iterate();
        }

        ui_state.visualization_update_needed = true;
    }
}

fn update_visualization(
    mut commands: Commands,
    mut ui_state: ResMut<UIState>,
    universe_state: Res<WolframsUniverse>,
) {
    if ui_state.visualization_update_needed {
        for age in ui_state.last_updated_age..universe_state.current_age {
            let current_state = &universe_state.current_state[age];
            let y = get_y_position(age, ui_state.cell_size, ui_state.container_height);

            current_state.iter().enumerate().for_each(|(index, state)| {
                let x = get_x_position(index, ui_state.cell_size, ui_state.container_width);
                let sprite_bundle = get_stateful_cell_sprite_bundle(state, x, y, &ui_state);

                commands.spawn_bundle(sprite_bundle).insert(Cell {
                    x,
                    y,
                    size: ui_state.cell_size,
                    state: *state,
                });
            });
        }

        ui_state.last_updated_age = universe_state.current_age;
        ui_state.visualization_update_needed = false;
    }
}

pub fn get_stateful_cell_sprite_bundle(
    state: &bool,
    x_position: f32,
    y_position: f32,
    ui_state: &UIState,
) -> SpriteBundle {
    if *state {
        get_cell_sprite_bundle(
            ui_state.full_color,
            x_position,
            y_position,
            ui_state.cell_size,
        )
    } else {
        get_cell_sprite_bundle(
            ui_state.empty_color,
            x_position,
            y_position,
            ui_state.cell_size,
        )
    }
}
