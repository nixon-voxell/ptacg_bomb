mod input;
mod procedural_generation;
mod rectangle;
mod rectangle_spawn;
mod spawner;

use bevy::prelude::*;
use input::{handle_player_input, PlayerAction};
use leafwing_input_manager::prelude::{ActionState, InputManagerPlugin};
use spawner::setup; // Only import setup

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputManagerPlugin::<PlayerAction>::default())
        .init_resource::<ActionState<PlayerAction>>()
        .insert_resource(PlayerAction::create_input_map())
        .add_systems(Startup, setup) // No changes needed here
        .add_systems(Update, handle_player_input)
        .run();
}
