use bevy::prelude::*;

pub struct PlayerPlugin;


#[derive(Component)]
pub struct Player{
    pub speed: f32,
}

impl Plugin for PlayerPlugin{
    fn build(&self, app:&mut App){
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, player_movement);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/car.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0), // Start at origin
                scale: Vec3::splat(1.0),              // Scale the sprite
                rotation: Quat::from_rotation_z(1.6),
                ..default()
            },
            ..default()
        },
        Player {speed: 300.0},
    ));

}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,  // New input system in Bevy 0.14
    time: Res<Time>,                      // Time resource for delta time
    mut query: Query<(&Player, &mut Transform)>,  // Query for player and its transform
) {
    for (player, mut transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        // Check for key presses and update direction
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        // Apply movement based on direction, player speed, and delta time
        let movement = direction.normalize_or_zero() * player.speed * time.delta_seconds();
        transform.translation += movement;
    }
}