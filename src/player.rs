use bevy::prelude::*;
use bevy_rapier2d::control::{KinematicCharacterController, KinematicCharacterControllerOutput};

use crate::state::ScheduleSet;
use crate::input::InputEvent;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            (
                gravity,
                move_horizontal,
                jump,
            ).in_set(ScheduleSet::MainUpdate),
            (
                reset_velocity_on_collision,
            ).in_set(ScheduleSet::VelocityCorrection),
            (
                apply_velocity,
            ).in_set(ScheduleSet::TransformUpdate),
        ));
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
    let delta_y = -400.0 * time.delta_seconds();
    for (mut velocity, character_controller) in query.iter_mut() {
        if !character_controller.grounded {
            velocity.0.y += delta_y;
        }
    }
}

pub fn move_horizontal(
    mut query: Query<&mut Velocity, With<Player>>,
    time: Res<Time>,
    mut input_events: EventReader<InputEvent>
) { 
    let acceleration = 500.0;
    let max_speed: f32 = 170.0;

    let mut active_movement = false;
    for event in input_events.read() {
        match event {
            InputEvent::MoveLeft(input_strength) => {
                for mut velocity in query.iter_mut() {
                    if velocity.0.x > 0.0 {
                        velocity.0.x = 0.0;
                    }
                    velocity.0.x = (-max_speed as f32 * input_strength).max(velocity.0.x - acceleration * time.delta_seconds());
                    active_movement = true;
                }
            }
            InputEvent::MoveRight(input_strength) => {
                for mut velocity in query.iter_mut() {
                    if velocity.0.x < 0.0 {
                        velocity.0.x = 0.0;
                    }
                    velocity.0.x = (max_speed * input_strength).min(velocity.0.x + acceleration * time.delta_seconds());
                    active_movement = true;
                }
            }
            _ => {}
        } 
    }
    if !active_movement {
        for mut velocity in query.iter_mut() {
            velocity.0.x = 0.0;
        }
    }
}

pub fn jump (
    mut query: Query<(&mut Velocity, &KinematicCharacterControllerOutput), With<Player>>,
    mut input_events: EventReader<InputEvent>
) {
    for event in input_events.read() {
        match event {
            InputEvent::Jump => {
                for (mut velocity, charachter_controller) in query.iter_mut() {
                    if charachter_controller.grounded {
                        velocity.0.y = 200.0;
                    }
                }
            }
            _ => {}
        }
    }
}