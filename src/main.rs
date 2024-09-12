mod input;

use bevy::prelude::*;
use input::{move_player, PlayerAction};
use leafwing_input_manager::prelude::{ActionState, InputManagerPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputManagerPlugin::<PlayerAction>::default())
        // Initialize the ActionState resource
        .init_resource::<ActionState<PlayerAction>>()
        // Insert the InputMap resource
        .insert_resource(PlayerAction::mkb_input_map())
        .add_systems(Update, move_player)
        .run();
}
