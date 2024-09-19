use bevy::{prelude::*, render::camera};
use blenvy::*;

use crate::player::Player;

pub struct BlenvyTestPlugin;

impl Plugin for BlenvyTestPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BlenvyPlugin::default())
            .register_type::<Player>()
            .register_type::<PlayerCamera>()
            .add_systems(Startup, setup)
            .add_systems(Update, camera_follow_player);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct PlayerCamera {
    follow_speed: f32,
}

fn setup(mut commands: Commands) {
    commands.spawn((
        BlueprintInfo::from_path("levels/World.glb"),
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
    ));
}

fn camera_follow_player(
    time: Res<Time>,
    mut q_transforms: Query<&mut Transform>,
    player_query: Query<Entity, With<Player>>, // Query to get the player's Transform
    camera_query: Query<(&PlayerCamera, Entity)>, // Query for the camera's Transform and CameraFollow component
) {
    let Ok((camera, camera_entity)) = camera_query.get_single() else {
        println!("no camera");
        return;
    };

    let Ok(player_entity) = player_query.get_single() else {
        println!("no player");
        return;
    };

    let Ok(player_transform) = q_transforms.get(player_entity).copied() else {
        println!("no player transform");
        return;
    };

    let Ok(mut camera_transform) = q_transforms.get_mut(camera_entity) else {
        println!("no camera transform");
        return;
    };

    println!("Camera Following!");

    camera_transform.translation = player_transform.translation;
    // if let Ok(player_transform) = player_query.get_single() {
    //     if let Ok((mut camera_transform, camera_follow)) = camera_query.get_single_mut() {
    //         // Calculate the difference between the player's position and the camera's position
    //         let player_position = player_transform.translation;
    //         let camera_position = camera_transform.translation;

    //         // Interpolate the camera's position toward the player's position using the follow speed
    //         let lerp_factor = camera_follow.follow_speed * time.delta_seconds();
    //         camera_transform.translation.x += (player_position.x - camera_position.x) * lerp_factor;
    //         camera_transform.translation.y += (player_position.y - camera_position.y) * lerp_factor;

    //         // Keep the camera at a fixed distance on the z-axis
    //         camera_transform.translation.z = 100.0;
    //     }
    // }
}
