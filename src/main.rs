mod input;
mod player;
mod spawn_camera;
mod blenvy;

use bevy::prelude::*;
use input::{handle_player_input, PlayerAction};
use leafwing_input_manager::prelude::{ActionState, InputManagerPlugin};
use player::PlayerPlugin;
use spawn_camera::CameraPlugin;
use blenvy::BlenvyPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // input.rs stuff
        .add_plugins(InputManagerPlugin::<PlayerAction>::default())
        .init_resource::<ActionState<PlayerAction>>()
        .insert_resource(PlayerAction::create_input_map())
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(BlenvyPlugin)
        .add_systems(Update, handle_player_input)
        .run();
}


