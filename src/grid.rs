use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_grid);
    }
}

fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for i in 0..150 {
        for j in 0..80 {
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Quad::new(Vec2::new(8.0, 8.0))))
                    .into(),
                transform: Transform::from_xyz((i - 75) as f32 * 8.0, (j - 40) as f32 * 8.0, 0.0),
                material: materials.add(ColorMaterial::from(Color::rgba(
                    255.0,
                    255.0,
                    255.0,
                    (i + j) as f32 / 230.0,
                ))),
                ..default()
            });
        }
    }
}
