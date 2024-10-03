use bevy::prelude::*;

use crate::{
    camera_controller::CameraControllerPlugin,
    constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
    world::WorldPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "3D TD".to_string(),
                    resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            WorldPlugin,
            CameraControllerPlugin,
        ))
        .add_systems(Startup, make_camera);
    }
}

#[derive(Component)]
struct GameCamera;

fn make_camera(mut commands: Commands) {
    commands.spawn((GameCamera, Camera3dBundle::default()));
}
