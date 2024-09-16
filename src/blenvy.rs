use bevy::prelude::*;
use blenvy::*;


pub struct BlenvyPlugin;

impl Plugin for BlenvyPlugin{
    fn build(&self, app:& mut App){
        app.register_type::<Player>()
            .add_systems(Startup,setup);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Player {
    strength: f32,
    perception: f32,
    endurance: f32,
    charisma: f32,
    intelligence: f32,
    agility: f32,
    luck: f32,
}

fn setup(mut commands: Commands) {
    commands.spawn((
        BlueprintInfo::from_path("levels/World.glb"),
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
    ));
}