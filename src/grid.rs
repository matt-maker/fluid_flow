use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

pub const GRID_CELL_SIZE: Vec2 = Vec2::new(8.0, 8.0);
pub const GRID_WIDTH: i32 = 150;
pub const GRID_HEIGHT: i32 = 80;

#[derive(Component, Debug)]
pub struct Grid;

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
pub struct SimCell;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_cells, spawn_grid));
        app.add_systems(Update, update_cells);
    }
}

fn spawn_cells(
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
                        (i + j) as f32 / 250.0,
                    ))),
                    ..default()
                },
                SimCell,
            ));
        }
    }
}

fn update_cells(
    query_simcells: Query<&mut Handle<ColorMaterial>, With<SimCell>>,
    query_gridvalues: Query<&GridValues, With<Grid>>,
    mut color_material: ResMut<Assets<ColorMaterial>>,
) {
    let grid_vec = query_gridvalues
        .get_single()
        .expect("Could not get grid values vector in update_cells in grid.rs");

    let mut counter: usize = 0;

    for query_color_material in &query_simcells {
        if let Some(material) = color_material.get_mut(query_color_material) {
            material.color = Color::rgba(
                grid_vec.grid_values_vec[counter],
                grid_vec.grid_values_vec[counter + 1],
                grid_vec.grid_values_vec[counter + 2],
                grid_vec.grid_values_vec[counter + 3],
            );
            counter += 4;
        }
    }
}

fn spawn_grid(mut commands: Commands) {
    commands.spawn((GridValues::new(Vec::new()), Grid));
}
