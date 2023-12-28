use crate::grid::{GridValues, GRID_HEIGHT, GRID_WIDTH};
use bevy::prelude::*;

pub struct SimulatePlugin;

impl Plugin for SimulatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, simulate);
    }
}

fn simulate(mut query: Query<&mut GridValues>) {
    for mut grid_value in query.iter_mut() {
        for i in 0..GRID_WIDTH {
            for j in 0..GRID_HEIGHT {
                for k in 0..3 {
                    grid_value.grid_values_vec[((4 * i * GRID_WIDTH) + (4 * j) + k) as usize] =
                        255.0;
                }
                grid_value.grid_values_vec[((4 * i * GRID_WIDTH) + (4 * j) + 3) as usize] =
                    (i + j) as f32 / 230.0;
            }
        }
        println!("Simulate Test");
    }
}
