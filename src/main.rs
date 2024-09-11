// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
mod spawn_camera;
mod player;
use spawn_camera::CameraPlugin;
use player::PlayerPlugin;

// TODO: fix release workflow namings
fn main() -> AppExit {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(CameraPlugin)
    .add_plugins(PlayerPlugin)
    .run()
}
