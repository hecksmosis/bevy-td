use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::select_tile::*;

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
    // TODO: Chunk loading
    for i in 0..10 {
        for j in 0..10 {
            let pos = Vec3::new(i as f32, 0.0, j as f32);
            commands
                .spawn((
                    SceneBundle {
                        scene: asset_server.load("floor.gltf#Scene0"),
                        transform: Transform::from_translation(pos * 16.0),
                        ..default()
                    },
                    Tile,
                    TileType::Floor,
                    Selected(false),
                ))
                .insert(Position(pos))
                .with_children(|children| {
                    children
                        .spawn(Collider::cuboid(8.0, 2.1, 8.0))
                        .insert(TransformBundle::from(Transform::from_xyz(-8.0, 2.0, -8.0)))
                        .insert((
                            On::<Pointer<Over>>::send_event::<TileHighlightEvent>(),
                            On::<Pointer<Click>>::send_event::<TileSelectEvent>(),
                        ));
                });
        }
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
) {
    hovered
        .read()
        .filter_map(|ev| q_collider.get(ev.0).ok())
        .filter_map(|p| q_position.get(p.get()).ok())
        .for_each(|Position(pos)| {
            println!("Hover: {:?}", pos);
        })
}
