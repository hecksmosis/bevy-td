use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{currency::Currency, world::Position};

pub struct SelectTilePlugin;

impl Plugin for SelectTilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TileSelectEvent>().add_systems(
            Update,
            (
                select_tile.run_if(on_event::<TileSelectEvent>()),
                upgrade_selected,
                log_selected,
            ),
        );
    }
}

#[derive(Component)]
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
    mut q_selected: Query<(Entity, &mut Selected)>,
) {
    clicked
        .read()
        .filter_map(|ev| q_collider.get(ev.0).ok())
        .for_each(|parent| {
            q_selected.iter_mut().for_each(|(ent, mut sel)| {
                sel.0 = ent == parent.get();
            });
        })
}

pub trait Level {
    fn next(self) -> Self;
    fn cost(&self) -> usize;
}

#[derive(Debug)]
pub struct FarmLevel(usize);

impl From<usize> for FarmLevel {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl Level for FarmLevel {
    fn next(mut self) -> Self {
        self.0 += 1;
        self
    }

    fn cost(&self) -> usize {
        self.0 * 5
    }
}

#[derive(Debug)]
pub struct TileLevel(usize);

impl From<usize> for TileLevel {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl Level for TileLevel {
    fn next(mut self) -> Self {
        self.0 += 1;
        self
    }

    fn cost(&self) -> usize {
        self.0 * 5
    }
}

#[derive(Component, Debug)]
pub enum TileType {
    Floor,
    Farm(FarmLevel),
    Tile(TileLevel),
}

impl TileType {
    fn get_level(&self) -> usize {
        match self {
            TileType::Tile(level) => level.0,
            TileType::Farm(level) => level.0,
            _ => 0,
        }
    }

    fn next_cost(&self) -> usize {
        match self {
            TileType::Tile(level) => level.cost(),
            TileType::Farm(level) => level.cost(),
            _ => 0,
        }
    }

    fn try_next_level(&mut self, currency_amount: usize) -> Option<usize> {
        let cost = self.next_cost();
        (currency_amount <= cost).then(|| {
            *self = match self {
                TileType::Tile(level) => TileType::Tile((level.0 + 1).into()),
                TileType::Farm(level) => TileType::Farm((level.0 + 1).into()),
                _ => TileType::Floor,
            };
            cost
        })
    }

    fn make_farm(&mut self, currency_amount: usize) -> i32 {
        match self {
            TileType::Floor => {
                *self = TileType::Farm(FarmLevel(1));
                self.next_cost() as i32
            }
            TileType::Farm(_) => {
                self.try_next_level(currency_amount);
                self.next_cost() as i32
            }
            TileType::Tile(level) => {
                let level_cost = level.cost() as i32;
                *self = TileType::Farm(FarmLevel(1));
                -level_cost / 2
            }
        }
    }

    pub(crate) fn tile(&self) -> Option<usize> {
        match self {
            TileType::Tile(l) => Some(l.0),
            _ => None,
        }
    }

    pub(crate) fn is_floor(&self) -> bool {
        matches!(self, TileType::Floor)
    }

    pub(crate) fn is_farm(&self) -> bool {
        matches!(self, TileType::Farm(_))
    }
}

pub fn upgrade_selected(
    mut q_selected: Query<(&Selected, &mut TileType, &mut Handle<Scene>)>,
    asset_server: Res<AssetServer>,
    keys: Res<ButtonInput<KeyCode>>,
    mut currency: ResMut<Currency>,
) {
    if !keys.any_just_pressed([KeyCode::KeyU, KeyCode::KeyF]) {
        return;
    }

    q_selected
        .iter_mut()
        .filter(|(sel, ..)| sel.0)
        .for_each(|(_, mut tile_type, mut scene)| {
            if keys.just_pressed(KeyCode::KeyU) && !tile_type.is_farm() {
                if let Some(cost) = tile_type.try_next_level(currency.wood) {
                    *tile_type = TileType::Tile(1.into());
                    *scene = asset_server.load("tower.gltf#Scene0");
                    currency.wood -= cost;
                }
            }
        })
}

pub fn log_selected(q_selected: Query<(&Selected, &Position)>) {
    q_selected
        .iter()
        .filter(|(sel, _)| sel.0)
        .for_each(|(_, pos)| println!("tile with position: {:?} was selected", pos))
}
