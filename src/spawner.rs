use crate::rectangle::RectangleConfig;
use crate::rectangle_spawn::{spawn_rectangle_grid, RectanglePool};
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rectangle_pool = initialize_rectangle_pool(1000); // Initialize rectangle pool

    spawn_light(&mut commands);
    spawn_camera(&mut commands);

    let rectangle_config = RectangleConfig::default_rectangle();
    let rows = 10;
    let cols = 100;

    spawn_rectangle_grid(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut rectangle_pool,
        rectangle_config,
        rows,
        cols,
    );

    // Log the available count in the pool
    println!(
        "Available rectangles in pool: {}",
        rectangle_pool.available_count()
    );
}

// Function to initialize the rectangle pool
fn initialize_rectangle_pool(max_size: usize) -> RectanglePool {
    let mut pool = RectanglePool::new(max_size);
    pool.preload(max_size); // Preload all rectangles
    pool
}

// Function to spawn light
fn spawn_light(commands: &mut Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

// Function to spawn camera
fn spawn_camera(commands: &mut Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(20.0, 20.0, 0.0)), // Keep Z at 0
        ..default()
    });
}
