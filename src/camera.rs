use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;   
use crate::player::Player;

use crate::state::ScheduleSet;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            spawn_camera,
        ))
        .add_systems(PostUpdate, (
            (
                move_camera,
                zoom_on_scroll,
            ).in_set(ScheduleSet::PostTransformUpdate),
        ));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)).with_scale(Vec3 { x: 0.4, y: 0.4, z: 1.0 }),
            ..Default::default()
        }
    );
}

fn zoom_on_scroll(
    mut ev_scroll: EventReader<MouseWheel>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    for ev in ev_scroll.read() {
        for mut transform in query.iter_mut() {
            transform.scale *= Vec3::new(1.0 - ev.y / 10.0, 1.0 - ev.y / 10.0, 1.0);
        }
    }
}

fn move_camera(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    for player_transform in player_query.iter() {
        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.translation = player_transform.translation + Vec3::new(-700.0, -450.0, 1.0);
        }
    }
}