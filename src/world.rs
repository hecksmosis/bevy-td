use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;
use itertools::iproduct;

use crate::{
    constants::{TEXTURE_MAP, TILE_SIZE, WORLD_SIZE},
    select_tile::*,
    util::NotifyQueue,
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world)
            .add_event::<TileHighlightEvent>()
            .add_systems(
                Update,
                (log_hover.run_if(on_event::<TileHighlightEvent>()),),
            );
    }
}

#[derive(Component, Debug)]
pub struct Position(Vec3);

#[derive(Component)]
pub struct Tile;

#[derive(Event, Debug)]
pub struct TileHighlightEvent(Entity);

impl From<ListenerInput<Pointer<Over>>> for TileHighlightEvent {
    fn from(value: ListenerInput<Pointer<Over>>) -> Self {
        TileHighlightEvent(value.target)
    }
}

fn spawn_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    // TODO: Randomize
    let centers = [Vec3::new(2.0, 0.0, 2.0), Vec3::new(8.0, 0.0, 8.0)];
    const RADIUS: f32 = 2.0;

    // Switch to hexagon three axis coordinate space
    for (i, j) in iproduct!(0..WORLD_SIZE, 0..WORLD_SIZE) {
        // Convert to hexagonal coordinates
        let pos = Vec3::new(i as f32, 0.0, j as f32);
        let tile_type = if centers.iter().any(|center| center.distance(pos) < RADIUS) {
            // Add more resources
            TileType::Resource(ResourceType::Wood)
        } else {
            TileType::Floor
        };
        let collider_height = tile_type.get_collider_height();
        commands
            .spawn((
                SceneBundle {
                    scene: asset_server.load(TEXTURE_MAP.get(&tile_type).cloned().unwrap()),
                    transform: Transform::from_translation(pos * 16.0),
                    ..default()
                },
                Tile,
                Selected(false),
                tile_type,
            ))
            .insert(Position(pos))
            .with_children(|children| {
                let collider = Collider::convex_hull(&[
                    Vec3::new(1.0, 0.0, 0.0) * 8.0,
                    Vec3::new(-1.0, 0.0, 0.0) * 8.0,
                    Vec3::new(0.5, 0.0, 0.75_f32.sqrt()) * 8.0,
                    Vec3::new(-0.5, 0.0, 0.75_f32.sqrt()) * 8.0,
                    Vec3::new(-0.5, 0.0, -(0.75_f32).sqrt()) * 8.0,
                    Vec3::new(0.5, 0.0, -(0.75_f32).sqrt()) * 8.0,
                    Vec3::new(1.0, 1.0, 0.0) * 8.0,
                    Vec3::new(-1.0, 1.0, 0.0) * 8.0,
                    Vec3::new(0.5, 1.0, 0.75_f32.sqrt()) * 8.0,
                    Vec3::new(-0.5, 1.0, 0.75_f32.sqrt()) * 8.0,
                    Vec3::new(-0.5, 1.0, -(0.75_f32).sqrt()) * 8.0,
                    Vec3::new(0.5, 1.0, -(0.75_f32).sqrt()) * 8.0,
                ])
                .unwrap();

                children
                    .spawn(collider)
                    .insert(TransformBundle::from(Transform::from_xyz(
                        -TILE_SIZE,
                        collider_height,
                        -TILE_SIZE,
                    )))
                    .insert((
                        On::<Pointer<Over>>::send_event::<TileHighlightEvent>(),
                        On::<Pointer<Click>>::send_event::<TileSelectEvent>(),
                    ));
            });
    }

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 50.0, 0.0).looking_at(-Vec3::Y, Vec3::X),
        ..default()
    });
}

fn log_hover(
    mut hovered: EventReader<TileHighlightEvent>,
    q_collider: Query<&Parent>,
    q_position: Query<&Position>,
    mut notify_queue: ResMut<NotifyQueue>,
) {
    // TODO: slight highlighting of tiles
    hovered
        .read()
        .filter_map(|ev| q_collider.get(ev.0).ok())
        .filter_map(|p| q_position.get(p.get()).ok())
        .for_each(|Position(pos)| notify_queue.push(format!("Tile selected: {:?}", pos)))
}
