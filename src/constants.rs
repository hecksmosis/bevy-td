use bevy::utils::HashMap;
use itertools::izip;
use lazy_static::lazy_static;

use crate::select_tile::{ResourceType, TileType};

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 512.0;
pub const TOWER_TS: f32 = 2.0;
pub const COST_SCALING: usize = 5;
pub const TILE_COST_SCALING: usize = 5;
pub const FARM_COST_SCALING: usize = 5;
pub const FARM_PRODUCTION: usize = 5;
pub const MODIFY_PENALTY: f32 = 1.5;
pub const WORLD_SIZE: usize = 20;
pub const TILE_SIZE: f32 = 8.0;

lazy_static! {
    pub static ref TEXTURE_MAP: HashMap<TileType, String> = izip![
        vec![
            TileType::Floor,
            TileType::Resource(ResourceType::Wood),
            TileType::Tile(1.into()),
            TileType::Farm(1.into(), ResourceType::Wood)
        ],
        vec![
            "floor.gltf#Scene0".to_owned(),
            "res_wood.gltf#Scene0".to_owned(),
            "tower.gltf#Scene0".to_owned(),
            "farm_wood.gltf#Scene0".to_owned(),
        ],
    ]
    .collect::<HashMap<_, _>>();
}
