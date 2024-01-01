use crate::grid::{Grid, GridS, GridV, GridValues, GRID_HEIGHT, GRID_WIDTH};
use bevy::prelude::*;

pub struct SimulatePlugin;

impl Plugin for SimulatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, pop_sim_vec);
        app.add_systems(PostUpdate, clean_up);
    }
}

fn pop_sim_vec(mut query: Query<&mut GridValues, With<Grid>>) {
    if let Ok(mut grid_vec) = query.get_single_mut() {
        for i in 0..GRID_WIDTH {
            for j in 0..GRID_HEIGHT {
                for _ in 0..3 {
                    grid_vec.grid_values_vec.push(100.0);
                }
                grid_vec.grid_values_vec.push((i + j) as f32 / 230.0);
            }
        }
    }
}

fn clean_up(mut query: Query<&mut GridValues, With<Grid>>) {
    let mut grid_vec = query
        .get_single_mut()
        .expect("could not find grid_vec in clean up");
    grid_vec.grid_values_vec.clear();
}

fn integrate(
    dt: &f32,
    gravity: &f32,
    query_grid_s: Query<&GridS, With<Grid>>,
    mut query_grid_v: Query<&mut GridV, With<Grid>>,
) {
    if let Ok(grid_s) = query_grid_s.get_single() {
        if let Ok(mut grid_v) = query_grid_v.get_single_mut() {
            for i in 1..GRID_WIDTH {
                for j in 1..GRID_HEIGHT - 1 {
                    if grid_s.grid_s_vec[(i * GRID_HEIGHT + j) as usize] != 0.0
                        && grid_s.grid_s_vec[(i * GRID_HEIGHT + (j - 1)) as usize] != 0.0
                    {
                        grid_v.grid_v_vec[(i * GRID_HEIGHT + j) as usize] += gravity * dt;
                    }
                }
            }
        }
    }
}
