use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world)
            .add_systems(Update, log_positions);
    }
}

#[derive(Component)]
pub struct Position(Vec3);

#[derive(Component)]
pub struct Tile;

fn spawn_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    // TODO: Chunk loading
    for i in 0..10 {
        for j in 0..10 {
            commands.spawn((Tile, Position(Vec3::new(i as f32, j as f32, 0.0))));
        }
    }
}

fn log_positions(positions: Query<&Position, With<Tile>>) {
    for Position(position) in &positions {
        println!("position: {:?}", position);
    }
}
