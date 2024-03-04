use bevy::{prelude::*, window::PresentMode};
use bevy_rapier2d::prelude::*;

mod camera;
mod world;
mod player;
mod state;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Platformer".into(),
                    resolution: (1000., 750.).into(),
                    // present_mode: PresentMode::Mailbox,
                    ..default()
                    }),
                ..default()
            }),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            // RapierDebugRenderPlugin::default(),
            world::WorldPlugin,
            camera::CameraPlugin,
            player::PlayerPlugin,
        ))
        .insert_resource(RapierConfiguration {
            ..Default::default()
        })
        .add_state::<state::GameState>()
        .run();
}