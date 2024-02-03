use crate::schedule::SimulationSet;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

pub const GRID_CELL_SIZE: Vec2 = Vec2::new(8.0, 8.0);
pub const GRID_WIDTH: i32 = 150;
pub const GRID_HEIGHT: i32 = 100;

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
pub struct GridU {
    pub grid_u_vec: Vec<f32>,
}

impl GridU {
    pub fn new(grid_u_vec: Vec<f32>) -> Self {
        Self { grid_u_vec }
    }
}

#[derive(Component, Debug)]
pub struct GridV {
    pub grid_v_vec: Vec<f32>,
}

impl GridV {
    pub fn new(grid_v_vec: Vec<f32>) -> Self {
        Self { grid_v_vec }
    }
}

#[derive(Component, Debug)]
pub struct GridnewU {
    pub grid_newu_vec: Vec<f32>,
}

impl GridnewU {
    pub fn new(grid_newu_vec: Vec<f32>) -> Self {
        Self { grid_newu_vec }
    }
}

#[derive(Component, Debug)]
pub struct GridnewV {
    pub grid_newv_vec: Vec<f32>,
}

impl GridnewV {
    pub fn new(grid_newv_vec: Vec<f32>) -> Self {
        Self { grid_newv_vec }
    }
}

#[derive(Component, Debug)]
pub struct GridP {
    pub grid_p_vec: Vec<f32>,
}

impl GridP {
    pub fn new(grid_p_vec: Vec<f32>) -> Self {
        Self { grid_p_vec }
    }
}

#[derive(Component, Debug)]
pub struct GridS {
    pub grid_s_vec: Vec<f32>,
}

impl GridS {
    pub fn new(grid_s_vec: Vec<f32>) -> Self {
        Self { grid_s_vec }
    }
}

#[derive(Component, Debug)]
pub struct GridM {
    pub grid_m_vec: Vec<f32>,
}

impl GridM {
    pub fn new(grid_m_vec: Vec<f32>) -> Self {
        Self { grid_m_vec }
    }
}

#[derive(Component, Debug)]
pub struct GridnewM {
    pub grid_newm_vec: Vec<f32>,
}

impl GridnewM {
    pub fn new(grid_newm_vec: Vec<f32>) -> Self {
        Self { grid_newm_vec }
    }
}

#[derive(Component, Debug)]
pub struct Scene {
    pub gravity: f32,
    pub dt: f32,
    pub num_iters: usize,
    pub density: f32,
    pub h: f32,
    pub over_relaxation: f32,
}

impl Scene {
    pub fn new(
        gravity: f32,
        dt: f32,
        num_iters: usize,
        density: f32,
        h: f32,
        over_relaxation: f32,
    ) -> Self {
        Self {
            gravity,
            dt,
            num_iters,
            density,
            h,
            over_relaxation,
        }
    }
}

#[derive(Bundle)]
pub struct GridBundle {
    pub grid_values: GridValues,
    pub grid_u: GridU,
    pub grid_v: GridV,
    pub grid_newu: GridnewU,
    pub grid_newv: GridnewV,
    pub grid_p: GridP,
    pub grid_s: GridS,
    pub grid_m: GridM,
    pub grid_newm: GridnewM,
    pub scene: Scene,
}

#[derive(Component, Debug)]
pub struct SimCell;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_cells, spawn_grid));
        app.add_systems(
            PostStartup,
            (
                pop_grid_s,
                pop_grid_v,
                pop_grid_u,
                pop_grid_p,
                pop_grid_m,
                pop_grid_values,
            ),
        );
        app.add_systems(Update, update_cells.in_set(SimulationSet::GridUpdate));
    }
}

fn spawn_cells(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Quad::new(GRID_CELL_SIZE)))
                        .into(),
                    transform: Transform::from_xyz(
                        (x - GRID_WIDTH / 2) as f32 * GRID_CELL_SIZE[0],
                        (y - GRID_HEIGHT / 2) as f32 * GRID_CELL_SIZE[1],
                        0.0,
                    ),
                    material: materials
                        .add(ColorMaterial::from(Color::rgba(255.0, 255.0, 255.0, 1.0))),
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
    if let Ok(grid_vec) = query_gridvalues.get_single() {
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
}

fn pop_grid_s(mut query: Query<&mut GridS, With<Grid>>) {
    if let Ok(mut grid_s) = query.get_single_mut() {
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                if x == 0 || y == 0 || y == GRID_HEIGHT - 1 {
                    grid_s.grid_s_vec.push(0.0); // solid
                } else {
                    grid_s.grid_s_vec.push(1.0); // liquid
                }
            }
        }
    }
}

fn pop_grid_v(mut query: Query<&mut GridV, With<Grid>>) {
    if let Ok(mut grid_v) = query.get_single_mut() {
        for _ in 0..GRID_WIDTH {
            for _ in 0..GRID_HEIGHT {
                grid_v.grid_v_vec.push(0.0);
            }
        }
    }
}

fn pop_grid_u(mut query: Query<&mut GridU, With<Grid>>) {
    if let Ok(mut grid_u) = query.get_single_mut() {
        for _ in 0..GRID_WIDTH {
            for _ in 0..GRID_HEIGHT {
                grid_u.grid_u_vec.push(0.0);
            }
        }
    }
}

fn pop_grid_p(mut query: Query<&mut GridP, With<Grid>>) {
    if let Ok(mut grid_p) = query.get_single_mut() {
        for _ in 0..GRID_WIDTH {
            for _ in 0..GRID_HEIGHT {
                grid_p.grid_p_vec.push(0.0);
            }
        }
    }
}

fn pop_grid_m(mut query: Query<&mut GridM, With<Grid>>) {
    if let Ok(mut grid_m) = query.get_single_mut() {
        for _ in 0..GRID_WIDTH {
            for _ in 0..GRID_HEIGHT {
                grid_m.grid_m_vec.push(0.0);
            }
        }
    }
}

fn pop_grid_values(mut query: Query<&mut GridValues, With<Grid>>) {
    if let Ok(mut grid_v) = query.get_single_mut() {
        for _ in 0..GRID_WIDTH {
            for _ in 0..GRID_HEIGHT {
                for _ in 0..4 {
                    grid_v.grid_values_vec.push(100.0);
                }
            }
        }
    }
}

fn spawn_grid(mut commands: Commands) {
    commands.spawn((
        GridBundle {
            grid_values: GridValues::new(Vec::new()), // keep for now
            grid_m: GridM::new(Vec::new()),
            grid_newm: GridnewM::new(Vec::new()),
            grid_newu: GridnewU::new(Vec::new()),
            grid_newv: GridnewV::new(Vec::new()),
            grid_p: GridP::new(Vec::new()),
            grid_s: GridS::new(Vec::new()),
            grid_u: GridU::new(Vec::new()),
            grid_v: GridV::new(Vec::new()),
            scene: Scene::new(
                9.81,       //gravity
                1.0 / 60.0, //dt
                40,         //num_iters
                1000.0,     //density
                0.01,       //h
                1.9,        //over_relaxation
            ),
        },
        Grid,
    ));
}
