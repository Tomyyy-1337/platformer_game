use bevy::prelude::*;

use bevy_rapier2d::prelude::*;

mod camera;
mod world;
mod player;
mod state;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
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