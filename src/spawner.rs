//use crate::procedural_generation::perlin_noise_cave::generate_perlin_noise_cave;
use crate::procedural_generation::random_walk_cave::generate_random_walk_cave;
use crate::rectangle::RectangleConfig;
use crate::rectangle_spawn::{spawn_rectangle_grid, RectanglePool};
use bevy::prelude::*;
use rand::Rng;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rectangle_pool = initialize_rectangle_pool(10000); // Initialize rectangle pool

    spawn_light(&mut commands);
    spawn_camera(&mut commands);

    let rectangle_config = RectangleConfig::default_rectangle();
    let width = 100;
    let height = 100;
    //let modifier = 0.1; // Adjust this value for noise smoothness
    //let edges_are_walls = true; // Set whether edges should be walls
    let seed: u32 = rand::thread_rng().gen_range(0..u32::MAX); // Generate a random seed

    //let map = generate_perlin_noise_cave(width, height, modifier, edges_are_walls, seed, 0.5, 1, 0);
    let map = generate_random_walk_cave(width, height, seed, 50.0);

    spawn_rectangle_grid(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut rectangle_pool,
        rectangle_config,
        map,
    );

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
        transform: Transform::from_translation(Vec3::new(15000.0, 30.0, 0.0)),
        projection: OrthographicProjection {
            scale: 50.0,
            near: -1000.0,
            ..default()
        },
        ..default()
    });
}
