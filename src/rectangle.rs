use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

// This struct holds a size (width or height) for a rectangle.
#[derive(Debug, Clone, Copy)]
pub struct Dimension(f32);

impl Dimension {
    pub fn new(value: f32) -> Option<Self> {
        if value > 0.0 {
            Some(Self(value))
        } else {
            None
        }
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

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

impl RectangleConfig {
    pub fn new(width: Dimension, height: Dimension, color: Color) -> Self {
        Self {
            width,
            height,
            color,
        }
    }

    pub fn default_rectangle() -> Self {
        Self::new(
            Dimension::new(100.0).unwrap(),
            Dimension::new(100.0).unwrap(),
            Color::srgb(0.0, 0.5, 0.8),
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
        create_rectangle_mesh(meshes, config.width.value(), config.height.value())?;
    commands.spawn(MaterialMesh2dBundle {
        mesh: rectangle_handle,
        material: materials.add(config.color),
        transform: position,
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
    let rectangle_mesh = Rectangle::new(width, height);
    let handle = meshes.add(rectangle_mesh);
    Ok(Mesh2dHandle(handle))
}
