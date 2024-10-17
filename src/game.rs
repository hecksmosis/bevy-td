use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_rapier3d::prelude::*;

use crate::{
    camera::CameraControllerPlugin,
    constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
    currency::CurrencyPlugin,
    select_tile::SelectTilePlugin,
    ui::UIPlugin,
    util::NotifyPlugin,
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
            SelectTilePlugin,
            DefaultPickingPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            CurrencyPlugin,
            UIPlugin,
            NotifyPlugin,
        ));
    }
}
