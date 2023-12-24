use bevy::prelude::*;

pub struct CameraPlugin;

#[derive(Debug, Component)]
struct MainCamera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle { ..default() }, MainCamera));
}
