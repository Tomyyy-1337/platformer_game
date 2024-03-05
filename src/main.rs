use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_asset_loader::prelude::*;

mod camera;
mod world;
mod player;
mod state;
mod menu;
mod asset_loader;

fn main() {
    App::new()
        .add_state::<state::AppState>()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Platformer".into(),
                        resolution: (1000., 750.).into(),
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
        ))
        .insert_resource(RapierConfiguration {
            ..Default::default()
        })
        .run();
}