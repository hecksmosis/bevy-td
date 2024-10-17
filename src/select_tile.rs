use std::ops::Deref;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    constants::{
        COST_SCALING, FARM_COST_SCALING, MODIFY_PENALTY, TEXTURE_MAP, TILE_COST_SCALING, TILE_SIZE,
    },
    currency::Currency,
    util::NotifyQueue,
    world::Position,
};

pub struct SelectTilePlugin;

impl Plugin for SelectTilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TileSelectEvent>().add_systems(
            Update,
            (
                select_tile.run_if(on_event::<TileSelectEvent>()),
                upgrade_selected,
            ),
        );
    }
}

#[derive(Component, Clone)]
pub struct Selected(pub bool);

#[derive(Event, Debug)]
pub struct TileSelectEvent(Entity);

impl From<ListenerInput<Pointer<Click>>> for TileSelectEvent {
    fn from(value: ListenerInput<Pointer<Click>>) -> Self {
        TileSelectEvent(value.target)
    }
}

pub fn select_tile(
    mut clicked: EventReader<TileSelectEvent>,
    q_collider: Query<&Parent>,
    mut q_selected: Query<(Entity, &mut Selected, &Position)>,
    mut notify_queue: ResMut<NotifyQueue>,
) {
    clicked
        .read()
        .filter_map(|ev| q_collider.get(ev.0).ok())
        .for_each(|parent| {
            q_selected.iter_mut().for_each(|(ent, mut sel, position)| {
                sel.0 = ent == parent.get();
                sel.0
                    .then(|| notify_queue.push(format!("Tile selected: {:?}", position)));
            });
        })
}

pub trait Level {
    fn next(&mut self);
    fn cost(&self) -> usize;
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct FarmLevel(usize);

impl From<usize> for FarmLevel {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl Level for FarmLevel {
    fn next(&mut self) {
        self.0 += 1;
    }

    fn cost(&self) -> usize {
        self.0 + 1
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct TileLevel(usize);

impl From<usize> for TileLevel {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl Level for TileLevel {
    fn next(&mut self) {
        self.0 += 1;
    }

    fn cost(&self) -> usize {
        (self.0 + 1) * COST_SCALING
    }
}

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub enum ResourceType {
    Wood,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    MakeFarm,
    MakeTile,
}

impl TryFrom<KeyCode> for Action {
    type Error = ();

    fn try_from(value: KeyCode) -> Result<Self, Self::Error> {
        match value {
            KeyCode::KeyF => Ok(Self::MakeFarm),
            KeyCode::KeyU => Ok(Self::MakeTile),
            _ => Err(()),
        }
    }
}

#[derive(Component, PartialEq, Debug, Eq, Hash, Clone)]
pub enum TileType {
    Floor,
    Resource(ResourceType),
    Farm(FarmLevel, ResourceType),
    Tile(TileLevel),
}

impl TileType {
    fn get_level(&self) -> usize {
        match self {
            TileType::Tile(level) => level.0,
            TileType::Farm(level, ..) => level.0,
            _ => 0,
        }
    }

    fn next_cost(&self, action: &Action) -> Option<usize> {
        match self {
            TileType::Tile(..) if action == &Action::MakeFarm => {
                Some((FARM_COST_SCALING as f32 * MODIFY_PENALTY).ceil() as usize)
            }
            TileType::Farm(..) if action == &Action::MakeTile => {
                Some((TILE_COST_SCALING as f32 * MODIFY_PENALTY).ceil() as usize)
            }
            TileType::Tile(level) if action == &Action::MakeTile => Some(level.cost()),
            TileType::Farm(level, _res) if action == &Action::MakeFarm => Some(level.cost()),
            _ => None,
        }
    }

    fn upgrade_cost(&self, action: &Action) -> usize {
        match self {
            TileType::Floor if action == &Action::MakeTile => TILE_COST_SCALING,
            // Implement different scaling for resource types
            TileType::Resource(_res) if action == &Action::MakeFarm => FARM_COST_SCALING,
            _ => 0,
        }
    }

    fn get_action_cost(&self, action: &Action) -> usize {
        self.next_cost(action).unwrap_or(self.upgrade_cost(action))
    }

    fn try_upgrade(&mut self, currency_amount: usize, key: KeyCode) -> Option<usize> {
        let Ok(action) = Action::try_from(key) else {
            return None;
        };
        let cost = self.get_action_cost(&action);
        info!("{}", cost);
        (currency_amount >= cost).then(|| {
            match self {
                TileType::Tile(ref mut level) if action == Action::MakeTile => level.next(),
                TileType::Farm(ref mut level, _) if action == Action::MakeFarm => level.next(),
                TileType::Resource(resource_type) if action == Action::MakeFarm => {
                    *self = TileType::Farm(FarmLevel(1), resource_type.clone());
                }
                TileType::Floor if action == Action::MakeTile => {
                    *self = TileType::Tile(TileLevel(1));
                }
                _ => {}
            };
            cost
        })
    }

    #[inline]
    pub(crate) fn farm(&self) -> Option<usize> {
        match self {
            TileType::Farm(l, _) => Some(l.0),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn tile(&self) -> Option<usize> {
        match self {
            TileType::Tile(l) => Some(l.0),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn is_floor(&self) -> bool {
        matches!(self, TileType::Floor)
    }

    #[inline]
    pub(crate) fn is_farm(&self) -> bool {
        matches!(self, TileType::Tile(_))
    }

    #[inline]
    pub(crate) fn is_tile(&self) -> bool {
        matches!(self, TileType::Farm(_, _))
    }

    pub(crate) fn get_collider_height(&self) -> f32 {
        match self {
            Self::Resource(ResourceType::Wood) => 10.1,
            Self::Tile(..) => 9.1,
            Self::Farm(..) => 4.1,
            _ => 2.1,
        }
    }
}

pub fn upgrade_selected(
    mut q_selected: Query<(&Selected, Entity, &mut TileType, &mut Handle<Scene>)>,
    mut q_collider: Query<(&mut Collider, &mut Transform, &Parent)>,
    asset_server: Res<AssetServer>,
    keys: Res<ButtonInput<KeyCode>>,
    mut currency: ResMut<Currency>,
) {
    currency.wood -= q_selected
        .iter_mut()
        .find(|(sel, ..)| sel.0)
        .and_then(|(_, entity, mut tile_type, mut scene)| {
            keys.get_just_pressed().next().and_then(|k| {
                tile_type.try_upgrade(currency.wood, *k).inspect(|_| {
                    *scene = asset_server.load(
                        TEXTURE_MAP
                            .get(tile_type.deref())
                            .expect("No texture for tile!"),
                    );

                    if let Some((mut collider, mut transform, ..)) =
                        q_collider.iter_mut().find(|(.., p)| p.get() == entity)
                    {
                        let height = tile_type.get_collider_height();
                        if let Some(mut cuboid) = collider.as_cuboid_mut() {
                            cuboid.set_half_extents((TILE_SIZE, height, TILE_SIZE).into());
                        }
                        transform.translation.y = height;
                    }
                })
            })
        })
        .unwrap_or(0)
}
