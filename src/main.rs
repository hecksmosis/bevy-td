use bevy::prelude::*;
use game::GamePlugin;

mod camera_controller;
mod constants;
mod game;
mod world;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
