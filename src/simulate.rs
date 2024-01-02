use crate::grid::{Dt, Gravity, Grid, GridS, GridV, GridValues, GRID_HEIGHT, GRID_WIDTH};
use bevy::prelude::*;

pub struct SimulatePlugin;

impl Plugin for SimulatePlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(Update, integrate);
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

fn clean_up(mut query: Query<(&mut GridValues, &mut GridV), With<Grid>>) {
    if let Ok((mut gridvalues, mut gridv)) = query.get_single_mut() {
        gridvalues.grid_values_vec.clear();
        gridv.grid_v_vec.clear();
    }
}

// Vectors need to be set to the required size and populated
fn integrate(
    query: Query<(&GridS, &Gravity, &Dt), With<Grid>>,
    mut query_grid_v: Query<&mut GridV, With<Grid>>,
) {
    if let Ok((grid_s, grav, time)) = query.get_single() {
        if let Ok(mut grid_v) = query_grid_v.get_single_mut() {
            for i in 1..GRID_WIDTH {
                for j in 1..GRID_HEIGHT {
                    if grid_s.grid_s_vec[(i * GRID_WIDTH + j) as usize] != 0.0
                        && grid_s.grid_s_vec[(i * GRID_WIDTH + (j - 1)) as usize] != 0.0
                    {
                        grid_v.grid_v_vec[(i * GRID_WIDTH + j) as usize] += grav.gravity * time.dt;
                    }
                    println!("{}", grid_v.grid_v_vec[3]);
                }
            }
        }
    }
}
