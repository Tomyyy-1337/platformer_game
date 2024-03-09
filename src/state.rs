use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ScheduleSet {
    CheckMenu,
    PauseMenu,
    HandleInput,
    MainUpdate,
    VelocityCorrection,
    TransformUpdate,
    SyncRapier,
    StepRapier,
    WritebackRapier,
    PostTransformUpdate,
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum AppState {
    #[default]
    Running,
    Menu,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>().configure_sets(
            Update,
            (
                (ScheduleSet::HandleInput,).run_if(in_state(AppState::Running)),
                ScheduleSet::CheckMenu,
                (
                    ScheduleSet::MainUpdate,
                    ScheduleSet::VelocityCorrection,
                    ScheduleSet::TransformUpdate,
                    ScheduleSet::SyncRapier,
                    ScheduleSet::StepRapier,
                    ScheduleSet::WritebackRapier,
                    ScheduleSet::PostTransformUpdate,
                )
                    .chain()
                    .run_if(in_state(AppState::Running)),
                (ScheduleSet::PauseMenu,).run_if(in_state(AppState::Menu)),
            )
                .chain(),
        ).add_systems(
            Update, (
                RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::SyncBackend).in_set(ScheduleSet::SyncRapier),
                RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::StepSimulation).in_set(ScheduleSet::StepRapier),
                RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::Writeback).in_set(ScheduleSet::WritebackRapier),
            )
        );
    }
}
