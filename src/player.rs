use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Player {
    pub speed: f32,
    pub rotation_damping: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .add_systems(Update, player_movement);
    }
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,                             // Time resource for delta time
    mut query: Query<(&Player, &mut Transform)>, // Query for player and its transform
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

        if direction.length_squared() > 0.0 {
            let target_angle = direction.y.atan2(direction.x); // Calculate target angle
            let mut current_angle = transform.rotation.to_euler(EulerRot::ZYX).0; // Get current angle

            // Smoothly damp the rotation towards the target angle
            current_angle = Quat::slerp(
                Quat::from_rotation_z(current_angle),
                Quat::from_rotation_z(target_angle),
                1.0 - (-player.rotation_damping * time.delta_seconds()).exp(),
            )
            .to_euler(EulerRot::ZYX)
            .0;

            transform.rotation = Quat::from_rotation_z(current_angle);
        }
    }
}
