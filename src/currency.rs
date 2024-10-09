use bevy::prelude::*;

use crate::{constants::TOWER_TS, select_tile::TileType, ui::CurrencyCounter};

pub struct CurrencyPlugin;

impl Plugin for CurrencyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Currency::default())
            .insert_resource(TowerTimer(Timer::from_seconds(
                TOWER_TS,
                TimerMode::Repeating,
            )))
            .add_systems(Update, (collect_towers, update_ui));
    }
}

#[derive(Resource)]
pub struct Currency {
    pub wood: usize,
}

impl Default for Currency {
    fn default() -> Self {
        Self { wood: 5 }
    }
}

#[derive(Resource)]
pub struct TowerTimer(Timer);

fn collect_towers(
    query: Query<&TileType>,
    mut currency: ResMut<Currency>,
    mut tower_timer: ResMut<TowerTimer>,
    time: Res<Time>,
) {
    if !tower_timer.0.tick(time.delta()).just_finished() {
        return;
    }
    query.iter().filter_map(|l| l.tile()).for_each(|tower| {
        currency.wood += 5 * tower;
    })
}

fn update_ui(currency: Res<Currency>, mut query: Query<&mut Text, With<CurrencyCounter>>) {
    let mut counter = query.single_mut();

    *counter = Text::from_section(format!("Money: {}", currency.wood), default());
}
