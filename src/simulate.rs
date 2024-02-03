use crate::grid::{
    Grid, GridM, GridP, GridS, GridU, GridV, GridValues, GridnewM, GridnewU, GridnewV, Scene,
    GRID_HEIGHT, GRID_WIDTH,
};
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
        app.add_systems(Update, advect_vel.in_set(SimulationSet::AdvectVel));
        app.add_systems(Update, advect_smoke.in_set(SimulationSet::AdvectSmoke));
        app.add_systems(
            Update,
            update_simulation_vector_values.in_set(SimulationSet::PopSimVec),
        );
    }
}

fn update_simulation_vector_values(mut query: Query<(&mut GridValues, &GridM), With<Grid>>) {
    if let Ok((mut grid_vec, grid_m)) = query.get_single_mut() {
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                /*grid_vec.grid_values_vec[((x * 4 * GRID_HEIGHT) + (y * 4)) as usize] =
                    255.0 * grid_m.grid_m_vec[((x * GRID_HEIGHT) + y) as usize];

                grid_vec.grid_values_vec[((x * 4 * GRID_HEIGHT) + (y * 4) + 1) as usize] =
                    255.0 * grid_m.grid_m_vec[((x * GRID_HEIGHT) + y) as usize];

                grid_vec.grid_values_vec[((x * 4 * GRID_HEIGHT) + (y * 4) + 2) as usize] =
                    255.0 * grid_m.grid_m_vec[((x * GRID_HEIGHT) + y) as usize];

                grid_vec.grid_values_vec[((x * 4 * GRID_HEIGHT) + (y * 4) + 3) as usize] = 1.0;*/

                //println!("{:?}", grid_m.grid_m_vec);

                // Issue is in the indexs above!!!
                grid_vec.grid_values_vec[((x * 4 * GRID_HEIGHT) + (y * 4)) as usize] = 100.0;
                grid_vec.grid_values_vec[((x * 4 * GRID_HEIGHT) + (y * 4) + 1) as usize] = 0.0;
                grid_vec.grid_values_vec[((x * 4 * GRID_HEIGHT) + (y * 4) + 2) as usize] = 0.0;
                grid_vec.grid_values_vec[((x * 4 * GRID_HEIGHT) + (y * 4) + 3) as usize] = 1.0;
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
            println!("Integrate grid_v {:?}", grid_v.grid_v_vec[102]);
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
            println!("SolveIncompressibility grid_v {:?}", grid_v.grid_v_vec[102]);
            // grid_v update from integrate carries to here

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
            println!(
                "SolveIncompressibility grid_v {}, grid_u {}, grid_p {} ",
                grid_v.grid_v_vec[102], grid_u.grid_u_vec[1000], grid_p.grid_p_vec[102]
            );
        }
    }
}

fn extrapolate(mut query_grid: Query<(&mut GridU, &mut GridV), With<Grid>>) {
    if let Ok((mut grid_u, mut grid_v)) = query_grid.get_single_mut() {
        println!(
            "Extrapolate grid_v {}, grid_u {} ",
            grid_v.grid_v_vec[102], grid_u.grid_u_vec[1000]
        );

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
        println!(
            "Extrapolate grid_v {}, grid_u {} ",
            grid_v.grid_v_vec[102], grid_u.grid_u_vec[1000]
        );
    }
}

fn advect_vel(
    query_scene: Query<&Scene, With<Grid>>,
    mut query_grid: Query<
        (&mut GridnewU, &mut GridU, &mut GridnewV, &mut GridV, &GridS),
        With<Grid>,
    >,
) {
    if let Ok(scene) = query_scene.get_single() {
        if let Ok((mut grid_new_u, mut grid_u, mut grid_new_v, mut grid_v, grid_s)) =
            query_grid.get_single_mut()
        {
            println!(
                "AdvectVel grid_v {}, grid_u {} ",
                grid_v.grid_v_vec[102], grid_u.grid_u_vec[1000]
            );

            grid_new_u.grid_newu_vec.clone_from(&grid_u.grid_u_vec);
            grid_new_v.grid_newv_vec.clone_from(&grid_v.grid_v_vec);

            println!(
                "AdvectVel grid_new_v {}, grid_new_u {} ",
                grid_new_v.grid_newv_vec[102], grid_new_u.grid_newu_vec[1000]
            );

            for x in 1..GRID_WIDTH {
                for y in 1..GRID_HEIGHT {
                    let _cnt: i32 = 1;

                    // u
                    if grid_s.grid_s_vec[(x * GRID_HEIGHT + y) as usize] != 0.0
                        && grid_s.grid_s_vec[((x - 1) * GRID_HEIGHT + y) as usize] != 0.0
                        && y < GRID_HEIGHT - 1
                    {
                        let mut var_x = x as f32 * scene.h;
                        let mut var_y = y as f32 * scene.h + (scene.h * 0.5);
                        let var_u = grid_u.grid_u_vec[(x * GRID_HEIGHT + y) as usize];
                        let var_v = (grid_v.grid_v_vec[((x - 1) * GRID_HEIGHT + y) as usize]
                            + grid_v.grid_v_vec[((x * GRID_HEIGHT) + y) as usize]
                            + grid_v.grid_v_vec[((x - 1) * GRID_HEIGHT + y + 1) as usize]
                            + grid_v.grid_v_vec[((x * GRID_HEIGHT) + y + 1) as usize])
                            * 0.25;
                        var_x = var_x - scene.dt * var_u;
                        var_y = var_y - scene.dt * var_v;

                        //U_FIELD samplefield function (sf variables)
                        let sf_x = scene.h.max(var_x.min(GRID_WIDTH as f32 * scene.h));
                        let sf_y = scene.h.max(var_y.min(GRID_HEIGHT as f32 * scene.h));
                        let mut sf_f: Vec<f32> = Vec::new();
                        sf_f.clone_from(&grid_u.grid_u_vec);
                        let sf_dy = scene.h * 0.5;
                        let x0 = ((GRID_WIDTH - 1) as f32).min(f32::floor(sf_x * (1.0 / scene.h)));
                        let tx = (sf_x - (x0 * scene.h)) * (1.0 / scene.h);
                        let x1 = ((GRID_WIDTH - 1) as f32).min(x0 + 1.0);
                        let y0 = ((GRID_HEIGHT - 1) as f32)
                            .min(f32::floor((sf_y - sf_dy) * (1.0 / scene.h)));
                        let ty = ((sf_y - sf_dy) - (y0 * scene.h)) * (1.0 / scene.h);
                        let y1 = ((GRID_HEIGHT - 1) as f32).min(y0 + 1.0);
                        let sx = 1.0 - tx;
                        let sy = 1.0 - ty;
                        let val = sx * sy * sf_f[(x0 * (GRID_HEIGHT as f32) + y0) as usize]
                            + tx * sy * sf_f[(x1 * (GRID_HEIGHT as f32) + y0) as usize]
                            + tx * ty * sf_f[(x1 * (GRID_HEIGHT as f32) + y1) as usize]
                            + sx * ty * sf_f[(x0 * (GRID_HEIGHT as f32) + y1) as usize];
                        grid_new_u.grid_newu_vec[(x * GRID_HEIGHT + y) as usize] = val;
                    }

                    //v
                    if grid_s.grid_s_vec[(x * GRID_HEIGHT + y) as usize] != 0.0
                        && grid_s.grid_s_vec[((x * GRID_HEIGHT) + (y - 1)) as usize] != 0.0
                        && x < GRID_WIDTH - 1
                    {
                        let mut var_x = x as f32 * scene.h + (scene.h * 0.5);
                        let mut var_y = y as f32 * scene.h;
                        let var_u = (grid_u.grid_u_vec[((x * GRID_HEIGHT) + (y - 1)) as usize]
                            + grid_u.grid_u_vec[((x * GRID_HEIGHT) + y) as usize]
                            + grid_u.grid_u_vec[((x + 1) * GRID_HEIGHT + (y - 1)) as usize]
                            + grid_u.grid_u_vec[((x + 1) * GRID_HEIGHT + y) as usize])
                            * 0.25;
                        let var_v = grid_v.grid_v_vec[(x * GRID_HEIGHT + y) as usize];
                        var_x = var_x - scene.dt * var_u;
                        var_y = var_y - scene.dt * var_v;

                        //V_FIELD samplefield function (sf variables)
                        let sf_x = scene.h.max(var_x.min(GRID_WIDTH as f32 * scene.h));
                        let sf_y = scene.h.max(var_y.min(GRID_HEIGHT as f32 * scene.h));
                        let mut sf_f: Vec<f32> = Vec::new();
                        sf_f.clone_from(&grid_v.grid_v_vec);
                        let sf_dx = scene.h * 0.5;
                        let x0 = ((GRID_WIDTH - 1) as f32)
                            .min(f32::floor((sf_x - sf_dx) * (1.0 / scene.h)));
                        let tx = ((sf_x - sf_dx) - (x0 * scene.h)) * (1.0 / scene.h);
                        let x1 = ((GRID_WIDTH - 1) as f32).min(x0 + 1.0);
                        let y0 = ((GRID_HEIGHT - 1) as f32).min(f32::floor(sf_y * (1.0 / scene.h)));
                        let ty = (sf_y - (y0 * scene.h)) * (1.0 / scene.h);
                        let y1 = ((GRID_HEIGHT - 1) as f32).min(y0 + 1.0);
                        let sx = 1.0 - tx;
                        let sy = 1.0 - ty;
                        let val = sx * sy * sf_f[(x0 * (GRID_HEIGHT as f32) + y0) as usize]
                            + tx * sy * sf_f[(x1 * (GRID_HEIGHT as f32) + y0) as usize]
                            + tx * ty * sf_f[(x1 * (GRID_HEIGHT as f32) + y1) as usize]
                            + sx * ty * sf_f[(x0 * (GRID_HEIGHT as f32) + y1) as usize];
                        grid_new_v.grid_newv_vec[(x * GRID_HEIGHT + y) as usize] = val;
                    }
                }
            }
            grid_u.grid_u_vec.clone_from(&grid_new_u.grid_newu_vec);
            grid_v.grid_v_vec.clone_from(&grid_new_v.grid_newv_vec);
            println!(
                "AdvectVel grid_v {}, grid_u {} ",
                grid_v.grid_v_vec[102], grid_u.grid_u_vec[1000]
            );
        }
    }
}

fn advect_smoke(
    scene_query: Query<&Scene, With<Grid>>,
    mut grid_query: Query<(&mut GridnewM, &mut GridM, &GridS, &GridU, &GridV), With<Grid>>,
) {
    if let Ok(scene) = scene_query.get_single() {
        if let Ok((mut grid_new_m, mut grid_m, grid_s, grid_u, grid_v)) =
            grid_query.get_single_mut()
        {
            println!(
                "AdvectSmoke grid_v {}, grid_u {}, grid_m {} ",
                grid_v.grid_v_vec[102], grid_u.grid_u_vec[1000], grid_m.grid_m_vec[102]
            );

            /*for i in (0..15000).step_by(100) {
                println!("{:?}", grid_m.grid_m_vec[i]);
            }*/

            grid_new_m.grid_newm_vec.clone_from(&grid_m.grid_m_vec);

            for x in 1..GRID_WIDTH - 1 {
                for y in 1..GRID_HEIGHT - 1 {
                    if grid_s.grid_s_vec[((x * GRID_HEIGHT) + y) as usize] != 0.0 {
                        let u = (grid_u.grid_u_vec[((x * GRID_HEIGHT) + y) as usize]
                            + grid_u.grid_u_vec[(((x + 1) * GRID_HEIGHT) + y) as usize])
                            * 0.5;
                        //println!("advect_smoke u {}", u);
                        let v = (grid_v.grid_v_vec[((x * GRID_HEIGHT) + y) as usize]
                            + grid_v.grid_v_vec[((x * GRID_HEIGHT) + (y + 1)) as usize])
                            * 0.5;
                        //println!("advect_smoke v {}", v);
                        let var_x = (x as f32 * scene.h) + (0.5 * scene.h) - (scene.dt * u);
                        //println!("advect_smoke var_x {}", var_x);
                        let var_y = (y as f32 * scene.h) + (0.5 * scene.h) - (scene.dt * v);
                        //println!("advect_smoke var_y {}", var_y);

                        //S_FIELD samplefield
                        let sf_x = scene.h.max(var_x.min(GRID_WIDTH as f32 * scene.h));
                        //println!("advect_smoke sf_x {}", sf_x);
                        let sf_y = scene.h.max(var_y.min(GRID_HEIGHT as f32 * scene.h));
                        //println!("advect_smoke sf_y {}", sf_y);
                        let mut sf_f: Vec<f32> = Vec::new();
                        sf_f.clone_from(&grid_m.grid_m_vec);
                        let sf_dx = scene.h * 0.5;
                        let sf_dy = scene.h * 0.5;
                        let x0 = ((GRID_WIDTH - 1) as f32)
                            .min(f32::floor((sf_x - sf_dx) * (1.0 / scene.h)));
                        //println!("advect_smoke x0 {}", x0);
                        let tx = ((sf_x - sf_dx) - (x0 * scene.h)) * (1.0 / scene.h);
                        //println!("advect_smoke tx {}", tx);
                        let x1 = ((GRID_WIDTH - 1) as f32).min(x0 + 1.0);
                        //println!("advect_smoke x1 {}", x1);
                        let y0 = ((GRID_HEIGHT - 1) as f32)
                            .min(f32::floor((sf_y - sf_dy) * (1.0 / scene.h)));
                        //println!("advect_smoke y0 {}", y0);
                        let ty = ((sf_y - sf_dy) - (y0 * scene.h)) * (1.0 / scene.h);
                        //println!("advect_smoke ty {}", ty);
                        let y1 = ((GRID_HEIGHT - 1) as f32).min(y0 + 1.0);
                        //println!("advect_smoke y1 {}", y1);
                        let sx = 1.0 - tx;
                        let sy = 1.0 - ty;
                        //println!("advect_smoke sx {} & sy {}", sx, sy);
                        let val = sx * sy * sf_f[(x0 * (GRID_HEIGHT as f32) + y0) as usize]
                            + tx * sy * sf_f[(x1 * (GRID_HEIGHT as f32) + y0) as usize]
                            + tx * ty * sf_f[(x1 * (GRID_HEIGHT as f32) + y1) as usize]
                            + sx * ty * sf_f[(x0 * (GRID_HEIGHT as f32) + y1) as usize];
                        println!("advect_smoke val {}", val);
                        // val=0, Issue is here as grid_m never updated to anything other than 0.0
                        grid_new_m.grid_newm_vec[((x * GRID_HEIGHT) + y) as usize] = val;
                    }
                }
            }
            grid_m.grid_m_vec.clone_from(&grid_new_m.grid_newm_vec);
            /*for i in (0..15000).step_by(100) {
                println!("{:?}", grid_m.grid_m_vec[i]);
            }*/
        }
    }
}
