use crate::rectangle::{spawn_rectangle, RectangleConfig};
use bevy::prelude::*;
use std::collections::VecDeque;

pub struct RectanglePool {
    available: VecDeque<RectangleConfig>,
    max_size: usize,
}

impl RectanglePool {
    pub fn new(max_size: usize) -> Self {
        Self {
            available: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    pub fn get(&mut self) -> Option<RectangleConfig> {
        self.available.pop_front()
    }

    #[allow(dead_code)]
    pub fn return_to_pool(&mut self, config: RectangleConfig) {
        if self.available.len() < self.max_size {
            self.available.push_back(config);
        }
    }

    pub fn preload(&mut self, count: usize) {
        for _ in 0..count {
            let config = RectangleConfig::default_rectangle();
            self.available.push_back(config);
        }
    }

    #[allow(dead_code)]
    pub fn resize(&mut self, new_size: usize) {
        if new_size > self.max_size {
            self.max_size = new_size;
            self.available.reserve(new_size - self.available.len());
            self.preload(new_size - self.available.len());
        }
    }

    pub fn available_count(&self) -> usize {
        self.available.len()
    }
}

// This function spawns a grid of rectangles based on a 2D array.
pub fn spawn_rectangle_grid(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    pool: &mut RectanglePool,
    config: RectangleConfig,
    map: Vec<Vec<i32>>,
) {
    let (rect_width, rect_height) = (config.width.value(), config.height.value());

    for (x, column) in map.iter().enumerate() {
        for (y, &value) in column.iter().enumerate() {
            if value == 1 {
                let position =
                    Transform::from_xyz(x as f32 * rect_width, y as f32 * rect_height, 0.0);
                if let Some(config) = pool.get() {
                    spawn_rectangle(commands, meshes, materials, config, position)
                        .expect("Failed to spawn rectangle");
                }
            }
        }
    }
}
