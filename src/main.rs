mod grid_spawning;
mod input;
mod procedural_generation;
mod rectangle_spawning;

use bevy::prelude::*;
use input::{handle_player_input, PlayerAction};
use leafwing_input_manager::prelude::{ActionState, InputManagerPlugin};

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(15000.0, 30.0, 0.0)),
        projection: OrthographicProjection {
            scale: 50.0,
            near: -1000.0,
            ..default()
        },
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputManagerPlugin::<PlayerAction>::default())
        .add_plugins(rectangle_spawning::rectangle_batch_spawn::RectangleBatchSpawnPlugin)
        .init_resource::<ActionState<PlayerAction>>()
        .insert_resource(PlayerAction::create_input_map())
        .add_systems(Startup, spawn_camera) // Correctly add the camera setup system without parentheses
        .add_systems(Update, handle_player_input)
        .run();
}
