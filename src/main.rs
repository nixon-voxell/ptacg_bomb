// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
mod spawn_camera;
use spawn_camera::CameraPlugin;

// TODO: fix release workflow namings
fn main() -> AppExit {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(CameraPlugin)
    .run()
}
