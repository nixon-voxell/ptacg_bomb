// This module manages a pool of rectangle objects to reuse them.
use crate::rectangle::{spawn_rectangle, RectangleConfig};
use bevy::prelude::*;
use std::collections::VecDeque;

// This struct holds a pool of rectangles that can be reused.
pub struct RectanglePool {
    available: VecDeque<RectangleConfig>, // Queue of available rectangles.
    max_size: usize,                      // Maximum number of rectangles in the pool.
}

impl RectanglePool {
    // Create a new rectangle pool with a set size.
    pub fn new(max_size: usize) -> Self {
        Self {
            available: VecDeque::with_capacity(max_size), // Prepare the queue.
            max_size,
        }
    }

    // Get a rectangle from the pool, if available.
    pub fn get(&mut self) -> Option<RectangleConfig> {
        self.available.pop_front() // Remove the first rectangle.
    }

    #[allow(dead_code)]
    // Return a rectangle to the pool to reuse it.
    pub fn return_to_pool(&mut self, config: RectangleConfig) {
        if self.available.len() < self.max_size {
            self.available.push_back(config); // Add back to the queue.
        }
    }

    // Preload rectangles into the pool.
    pub fn preload(&mut self, count: usize) {
        for _ in 0..count {
            let config = RectangleConfig::default_rectangle(); // Create default rectangle.
            self.available.push_back(config); // Add to pool.
        }
    }

    #[allow(dead_code)]
    // Dynamically resize the pool if needed.
    pub fn resize(&mut self, new_size: usize) {
        if new_size > self.max_size {
            self.max_size = new_size;
            self.available.reserve(new_size - self.available.len()); // Reserve additional space.
            self.preload(new_size - self.available.len()); // Preload new rectangles.
        }
    }

    // Check how many rectangles are available.
    pub fn available_count(&self) -> usize {
        self.available.len() // Return the count.
    }
}

// This function spawns a grid of rectangles.
pub fn spawn_rectangle_grid(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    pool: &mut RectanglePool,
    config: RectangleConfig,
    rows: usize,
    cols: usize,
) {
    let (rect_width, rect_height) = (config.width.value(), config.height.value()); // Get sizes.
    let positions = calculate_positions(rows, cols, rect_width, rect_height); // Calculate positions.
    batch_spawn_rectangles(commands, meshes, materials, pool, positions); // Spawn rectangles.
}

// This function spawns multiple rectangles at once.
fn batch_spawn_rectangles(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    pool: &mut RectanglePool,
    positions: Vec<Transform>,
) {
    for position in positions {
        if pool.available_count() > 0 {
            if let Some(config) = pool.get() {
                spawn_rectangle(commands, meshes, materials, config, position)
                    .expect("Failed to spawn rectangle");
            }
        } else {
            println!("Maximum pool size reached, cannot spawn more rectangles.");
            break; // Stop spawning if the pool is empty
        }
    }
}

// This function calculates where to place each rectangle in a grid.
fn calculate_positions(rows: usize, cols: usize, width: f32, height: f32) -> Vec<Transform> {
    let mut positions = Vec::with_capacity(rows * cols); // Preallocate memory for efficiency.
    let width_f = width; // Store values to avoid multiple calls.
    let height_f = height;

    for row in 0..rows {
        let y_position = row as f32 * height_f; // Calculate Y position once per row.
        for col in 0..cols {
            let x_position = col as f32 * width_f; // Calculate X position.
            positions.push(Transform::from_xyz(x_position, y_position, 0.0));
        }
    }

    positions // Return the precomputed positions.
}
