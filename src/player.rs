use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app:&mut App){
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(mut commands:Commands, asset_server: Res<AssetServer>){
    commands.spawn(SpriteBundle{
        texture: asset_server.load("images/car.png"),
        ..default()
    });

}
