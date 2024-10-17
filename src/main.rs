use bevy::prelude::*;
use game::GamePlugin;

mod camera;
mod constants;
mod currency;
mod game;
mod select_tile;
mod ui;
mod util;
mod world;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
