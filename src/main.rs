use bevy::prelude::*;

mod camera_2d;
mod grid;
mod schedule;
mod simulate;

use camera_2d::CameraPlugin;
use grid::GridPlugin;
use schedule::SchudulePlugin;
use simulate::SimulatePlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(SimulatePlugin)
        .add_plugins(GridPlugin)
        .add_plugins(SchudulePlugin)
        .run();
}
