use bevy::prelude::*;
use bevy::reflect::Enum;
use bevy::sprite::MaterialMesh2dBundle;

pub const GRID_CELL_SIZE: Vec2 = Vec2::new(8.0, 8.0);
pub const GRID_WIDTH: i32 = 150;
pub const GRID_HEIGHT: i32 = 80;

#[derive(Component, Debug)]
pub struct GridValues {
    pub grid_values_vec: Vec<f32>,
}

impl GridValues {
    pub fn new(grid_values_vec: Vec<f32>) -> Self {
        Self { grid_values_vec }
    }
}

#[derive(Component, Debug)]
pub struct SimGrid;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_grid);
        app.add_systems(Update, update_grid);
    }
}

fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for i in 0..GRID_WIDTH {
        for j in 0..GRID_HEIGHT {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Quad::new(GRID_CELL_SIZE)))
                        .into(),
                    transform: Transform::from_xyz(
                        (i - GRID_WIDTH / 2) as f32 * GRID_CELL_SIZE[0],
                        (j - GRID_HEIGHT / 2) as f32 * GRID_CELL_SIZE[1],
                        0.0,
                    ),
                    material: materials.add(ColorMaterial::from(Color::rgba(
                        255.0,
                        255.0,
                        255.0,
                        (i + j) as f32 / 230.0,
                    ))),
                    ..default()
                },
                GridValues::new(vec![]),
                SimGrid,
            ));
        }
    }
}

fn update_grid(
    query: Query<&mut Handle<ColorMaterial>, With<SimGrid>>,
    mut color_material: ResMut<Assets<ColorMaterial>>,
) {
    for query_color_material in &query {
        if let Some(material) = color_material.get_mut(query_color_material) {
            material.color = Color::RED;
            break;
        }
    }
}
