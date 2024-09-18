use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

// This struct holds a size (width or height) for a rectangle.
#[derive(Debug, Clone, Copy)]
pub struct Dimension(f32);

// Create a new dimension if the value is positive.
impl Dimension {
    pub fn new(value: f32) -> Option<Self> {
        if value > 0.0 {
            Some(Self(value)) // Return the new dimension.
        } else {
            None // Not allowed if it's zero or negative.
        }
    }

    // Get the value of the dimension.
    pub fn value(&self) -> f32 {
        self.0
    }
}

// Default dimension is 1.0.
impl Default for Dimension {
    fn default() -> Self {
        Self(1.0)
    }
}

// This struct holds the settings for a rectangle (size and color).
#[derive(Debug, Default, Clone)]
pub struct RectangleConfig {
    pub width: Dimension,
    pub height: Dimension,
    pub color: Color,
}

// Create a new rectangle config with width, height, and color.
impl RectangleConfig {
    pub fn new(width: Dimension, height: Dimension, color: Color) -> Self {
        Self {
            width,
            height,
            color,
        }
    }

    // Create a default rectangle with specific size and color.
    pub fn default_rectangle() -> Self {
        Self::new(
            Dimension::new(100.0).unwrap(), // Width of 100
            Dimension::new(100.0).unwrap(), // Height of 100
            Color::srgb(0.0, 0.5, 0.8),     // Blueish color
        )
    }
}

// This function spawns a rectangle in the game.
pub fn spawn_rectangle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    config: RectangleConfig,
    position: Transform,
) -> Result<(), String> {
    let rectangle_handle =
        create_rectangle_mesh(meshes, config.width.value(), config.height.value())?; // Create the rectangle mesh.
    commands.spawn(MaterialMesh2dBundle {
        mesh: rectangle_handle,
        material: materials.add(config.color), // Set the color.
        transform: position,                   // Set the position.
        ..default()
    });
    Ok(())
}

// This function creates a rectangle mesh for the game.
fn create_rectangle_mesh(
    meshes: &mut ResMut<Assets<Mesh>>,
    width: f32,
    height: f32,
) -> Result<Mesh2dHandle, String> {
    let rectangle_mesh = Rectangle::new(width, height); // Create rectangle mesh.
    let handle = meshes.add(rectangle_mesh); // Add mesh to assets.
    Ok(Mesh2dHandle(handle)) // Return the handle.
}
