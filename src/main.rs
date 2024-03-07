use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod camera;
mod world;
mod player;
mod state;
mod menu;
mod asset_loader;
mod input;

fn main() {
    App::new()
        .add_state::<state::AppState>()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Platformer".into(),
                        resolution: (1200., 750.).into(),
                        ..default()
                        }),
                    ..default()
                }
            ),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            // RapierDebugRenderPlugin::default(),
            asset_loader::AssetLoaderPlugin,
            world::WorldPlugin,
            camera::CameraPlugin,
            player::PlayerPlugin,
            state::SchedulePlugin,
            menu::MenuPlugin,
            input::InputPlugin,
        ))
        .insert_resource(RapierConfiguration {
            ..Default::default()
        })
        .run();
}