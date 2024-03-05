use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ScheduleSet {
    CheckMenu,
    PauseMenu,
    MainUpdate,
    VelocityCorrection,
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
        app.add_state::<AppState>()
            .configure_sets(Update, (
                ScheduleSet::CheckMenu,
                (
                    ScheduleSet::MainUpdate,
                    ScheduleSet::VelocityCorrection,
                    ScheduleSet::PostTransformUpdate,
                ).chain().run_if(in_state(AppState::Running)),
                ScheduleSet::PauseMenu.run_if(in_state(AppState::Menu))
            ).chain());
    }
}

