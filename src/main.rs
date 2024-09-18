mod blenvy;
mod input;
mod player;
mod spawn_camera;

use bevy::prelude::*;
use blenvy::BlenvyTestPlugin;
use input::{handle_player_input, PlayerAction};
use leafwing_input_manager::prelude::{ActionState, InputManagerPlugin};
use player::PlayerPlugin;
use spawn_camera::CameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // input.rs stuff
        .add_plugins(InputManagerPlugin::<PlayerAction>::default())
        .init_resource::<ActionState<PlayerAction>>()
        .insert_resource(PlayerAction::create_input_map())
        //.add_plugins(PlayerPlugin)
        //.add_plugins(CameraPlugin)
        .add_plugins(BlenvyTestPlugin)
        .add_systems(Update, handle_player_input)
        .run();
}
