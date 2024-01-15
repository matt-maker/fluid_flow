use crate::grid::{Grid, GridP, GridS, GridU, GridV, GridValues, Scene, GRID_HEIGHT, GRID_WIDTH};
use crate::schedule::SimulationSet;
use bevy::prelude::*;

pub struct SimulatePlugin;

impl Plugin for SimulatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, integrate.in_set(SimulationSet::Integrate));
        app.add_systems(
            Update,
            solve_incompressibility.in_set(SimulationSet::SolveIncompressibility),
        );
        app.add_systems(Update, extrapolate.in_set(SimulationSet::Extrapolate));
        app.add_systems(
            Update,
            update_simulation_vector_values.in_set(SimulationSet::PopSimVec),
        );
    }
}

fn update_simulation_vector_values(mut query: Query<&mut GridValues, With<Grid>>) {
    if let Ok(mut grid_vec) = query.get_single_mut() {
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                grid_vec.grid_values_vec[((x * 4 * GRID_HEIGHT) + (y * 4)) as usize] = 255.0;
                grid_vec.grid_values_vec[((x * 4 * GRID_HEIGHT) + (y * 4) + 1) as usize] = 0.0;
                grid_vec.grid_values_vec[((x * 4 * GRID_HEIGHT) + (y * 4) + 2) as usize] = 0.0;
                grid_vec.grid_values_vec[((x * 4 * GRID_HEIGHT) + (y * 4) + 3) as usize] = 0.5;
            }
        }
    }
}

fn integrate(
    query: Query<(&GridS, &Scene), With<Grid>>,
    mut query_grid_v: Query<&mut GridV, With<Grid>>,
) {
    if let Ok((grid_s, scene)) = query.get_single() {
        if let Ok(mut grid_v) = query_grid_v.get_single_mut() {
            for x in 1..GRID_WIDTH {
                for y in 1..GRID_HEIGHT - 1 {
                    if grid_s.grid_s_vec[(x * GRID_HEIGHT + y) as usize] != 0.0
                        && grid_s.grid_s_vec[(x * GRID_HEIGHT + (y - 1)) as usize] != 0.0
                    {
                        grid_v.grid_v_vec[(x * GRID_HEIGHT + y) as usize] +=
                            scene.gravity * scene.dt;
                    }
                }
            }
        }
    }
}

fn solve_incompressibility(
    query_scene: Query<&Scene, With<Grid>>,
    mut query_grid: Query<(&GridS, &mut GridU, &mut GridV, &mut GridP), With<Grid>>,
) {
    if let Ok(scene) = query_scene.get_single() {
        if let Ok((grid_s, mut grid_u, mut grid_v, mut grid_p)) = query_grid.get_single_mut() {
            let cp: f32 = scene.density * (scene.h / scene.dt);

            for _ in 0..scene.num_iters {
                for x in 1..GRID_WIDTH - 1 {
                    for y in 1..GRID_HEIGHT - 1 {
                        if grid_s.grid_s_vec[(x * GRID_HEIGHT + y) as usize] == 0.0 {
                            continue;
                        }

                        let mut s: f32 = grid_s.grid_s_vec[(x * GRID_HEIGHT + y) as usize];
                        let sx0: f32 = grid_s.grid_s_vec[((x - 1) * GRID_HEIGHT + y) as usize];
                        let sx1: f32 = grid_s.grid_s_vec[((x + 1) * GRID_HEIGHT + y) as usize];
                        let sy0: f32 = grid_s.grid_s_vec[(x * GRID_HEIGHT + (y - 1)) as usize];
                        let sy1: f32 = grid_s.grid_s_vec[(x * GRID_HEIGHT + (y + 1)) as usize];
                        s = sx0 + sx1 + sy0 + sy1;

                        if s == 0.0 {
                            continue;
                        }

                        let div: f32 = grid_u.grid_u_vec[((x + 1) * GRID_HEIGHT + y) as usize]
                            - grid_u.grid_u_vec[(x * GRID_HEIGHT + y) as usize]
                            + grid_v.grid_v_vec[(x * GRID_HEIGHT + y + 1) as usize]
                            - grid_v.grid_v_vec[(x * GRID_HEIGHT + y) as usize];

                        let mut p: f32 = -div / s;
                        p *= scene.over_relaxation;
                        grid_p.grid_p_vec[(x * GRID_HEIGHT + y) as usize] += cp * p;

                        grid_u.grid_u_vec[(x * GRID_HEIGHT + y) as usize] -= sx0 * p;
                        grid_u.grid_u_vec[((x + 1) * GRID_HEIGHT + y) as usize] += sx1 * p;
                        grid_v.grid_v_vec[(x * GRID_HEIGHT + y) as usize] -= sy0 * p;
                        grid_v.grid_v_vec[(x * GRID_HEIGHT + (y + 1)) as usize] += sy1 * p;
                    }
                }
            }
        }
    }
}

fn extrapolate(mut query_grid: Query<(&mut GridU, &mut GridV), With<Grid>>) {
    if let Ok((mut grid_u, mut grid_v)) = query_grid.get_single_mut() {
        for x in 0..GRID_WIDTH {
            grid_u.grid_u_vec[(x * GRID_HEIGHT + 0) as usize] =
                grid_u.grid_u_vec[(x * GRID_HEIGHT + 1) as usize];

            grid_u.grid_u_vec[((x * GRID_HEIGHT) + (GRID_HEIGHT - 1)) as usize] =
                grid_u.grid_u_vec[((x * GRID_HEIGHT) + (GRID_HEIGHT - 2)) as usize];
        }

        for y in 0..GRID_HEIGHT {
            grid_v.grid_v_vec[(0 * GRID_HEIGHT + y) as usize] =
                grid_v.grid_v_vec[(1 * GRID_HEIGHT + y) as usize];

            grid_v.grid_v_vec[(((GRID_WIDTH - 1) * GRID_HEIGHT) + y) as usize] =
                grid_v.grid_v_vec[(((GRID_WIDTH - 2) * GRID_HEIGHT) + y) as usize];
        }
    }
}
