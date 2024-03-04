use bevy::prelude::*;
use bevy_rapier2d::control::{KinematicCharacterController, KinematicCharacterControllerOutput};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, reset_velocity_on_collision)
            .add_systems(Update, (
                apply_velocity,
                gravity,
                move_horizontal,
                jump,
            ).before(reset_velocity_on_collision));
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Component, Clone, Debug, Default)]
pub struct Velocity(pub Vec2);

pub fn reset_velocity_on_collision(
    mut query: Query<(&mut Velocity, &KinematicCharacterControllerOutput), With<Player>>,
) {
    for (mut velocity, charachter_controller) in query.iter_mut() {
        for c in charachter_controller.collisions.iter() {
            match c.toi.details {
                Some(v) => {
                    if v.normal1.y < -0.5 {
                        velocity.0.y = 0.0;
                    }
                    if v.normal1.y > 0.5 && velocity.0.y < 0.0 {
                        velocity.0.y = 0.0;
                    }
                    if v.normal1.x.abs() > 0.7 {
                        velocity.0.x = 0.0;
                    }
                }
                _ => {}
            };
        }
    }
}

pub fn apply_velocity(
    velocity_query: Query<&mut Velocity, With<Player>>,
    mut charachter_controller_query: Query<&mut KinematicCharacterController, With<Player>>,
    time: Res<Time>,
) {
    for velocity in velocity_query.iter() {
        for mut charachter_controller in charachter_controller_query.iter_mut() {
            match charachter_controller.translation {
                Some(translation) => {
                    charachter_controller.translation = Some(translation + velocity.0 * time.delta_seconds());
                }
                None => {
                    charachter_controller.translation = Some(velocity.0 * time.delta_seconds());
                }
            }
        }
    }
}

pub fn gravity(
    mut query: Query<(&mut Velocity, &KinematicCharacterControllerOutput), With<Player>>,
    time: Res<Time>,
) {
    let delta_y = -200.0 * time.delta_seconds();
    for (mut velocity, character_controller) in query.iter_mut() {
        if !character_controller.grounded {
            velocity.0.y += delta_y;
        }
    }
}

pub fn move_horizontal(
    mut query: Query<&mut Velocity, With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) { 
    let acceleration = 300.0;
    let max_speed: f32 = 120.0;
    for mut velocity in query.iter_mut() {
        if input.pressed(KeyCode::A) {
            velocity.0.x = (-max_speed as f32).max(velocity.0.x - acceleration * time.delta_seconds());
        } else if input.pressed(KeyCode::D) {
            velocity.0.x = max_speed.min(velocity.0.x + acceleration * time.delta_seconds());
        } else if velocity.0.x > 0.0 {
            velocity.0.x = (velocity.0.x - acceleration * time.delta_seconds()).max(0.0);
        } else if velocity.0.x < 0.0 {
            velocity.0.x = (velocity.0.x + acceleration * time.delta_seconds()).min(0.0);
        }
    }
}

pub fn jump (
    mut query: Query<(&mut Velocity, &KinematicCharacterControllerOutput), With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        for (mut velocity, charachter_controller) in query.iter_mut() {
            if charachter_controller.grounded {
                velocity.0.y = 160.0;
            }
        }
    } 
}