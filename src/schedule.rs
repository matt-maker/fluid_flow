use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum SimulationSet {
    Integrate,
    SolveIncompressibility,
    Extrapolate,
    AdvectVel,
    AdvectSmoke,
    PopSimVec,
    GridUpdate,
}

pub struct SchudulePlugin;

impl Plugin for SchudulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                SimulationSet::Integrate,
                SimulationSet::SolveIncompressibility,
                SimulationSet::Extrapolate,
                SimulationSet::AdvectVel,
                SimulationSet::AdvectSmoke,
                SimulationSet::PopSimVec,
                //flush
                SimulationSet::GridUpdate,
            )
                .chain(),
        )
        .add_systems(
            Update,
            apply_deferred
                .after(SimulationSet::PopSimVec)
                .before(SimulationSet::GridUpdate),
        );
    }
}
