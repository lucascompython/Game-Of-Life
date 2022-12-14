use bevy::prelude::*;
use ui::MainMenuPlugin;
use input::InputPlugin;
use simulation::SimulationPlugin;

const GRID_SIZE: i32 = 100;

mod ui;
mod simulation;
mod input;
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1024f32,
            height: 720f32,
            title: String::from("Game Of Life"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(MainMenuPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(SimulationPlugin)
        .run();
}
