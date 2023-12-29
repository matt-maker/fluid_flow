use crate::grid::{Grid, GridValues, GRID_HEIGHT, GRID_WIDTH};
use bevy::prelude::*;

pub struct SimulatePlugin;

impl Plugin for SimulatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, simulate);
        app.add_systems(PostUpdate, clean_up);
    }
}

fn simulate(mut query: Query<&mut GridValues, With<Grid>>) {
    let mut grid_vec = query
        .get_single_mut()
        .expect("could not find grid_vec in simulate");
    for i in 0..GRID_WIDTH {
        for j in 0..GRID_HEIGHT {
            for _ in 0..3 {
                grid_vec.grid_values_vec.push(100.0);
            }
            grid_vec.grid_values_vec.push((i + j) as f32 / 230.0);
        }
    }
}

fn clean_up(mut query: Query<&mut GridValues, With<Grid>>) {
    let mut grid_vec = query
        .get_single_mut()
        .expect("could not find grid_vec in clean up");
    grid_vec.grid_values_vec.clear();
}
