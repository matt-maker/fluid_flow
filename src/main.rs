use bevy::prelude::*;

mod camera_2d;
mod grid;

use camera_2d::CameraPlugin;
use grid::GridPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(GridPlugin)
        .run();
}
