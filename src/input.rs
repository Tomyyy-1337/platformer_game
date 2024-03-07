use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;

use crate::state::ScheduleSet;

#[derive(Debug, Event)]
pub enum InputEvent {
    Zoom(f32),
    MoveLeft(f32),
    MoveRight(f32),
    Jump,
    ResetLevel,
    Menu,
    ToggleFullscreen,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputEvent>()
            .add_systems(Update, (
                handle_keyboard_input,
                handle_mouse_wheel_input,
                gamepad_system,
            ).in_set(ScheduleSet::HandleInput)
        );
    }
}

fn handle_mouse_wheel_input(
    mut input_event: EventWriter<InputEvent>,
    mut ev_scroll: EventReader<MouseWheel>,
) {
    for ev in ev_scroll.read() {
        input_event.send(InputEvent::Zoom(ev.y));
    }
}

fn handle_keyboard_input(
    mut input_event: EventWriter<InputEvent>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    keyboard_input.get_pressed().filter_map(|key| match key {
        KeyCode::A => Some(InputEvent::MoveLeft(1.0)),
        KeyCode::D => Some(InputEvent::MoveRight(1.0)),
        KeyCode::Space => Some(InputEvent::Jump),
        _ => None,
    }).for_each(|event| input_event.send(event));
    keyboard_input.get_just_pressed().filter_map(|key| match key {
        KeyCode::R => Some(InputEvent::ResetLevel),
        KeyCode::Escape => Some(InputEvent::Menu),
        KeyCode::F11 => Some(InputEvent::ToggleFullscreen),
        _ => None,
    }).for_each(|event| input_event.send(event));
}

fn gamepad_system(
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    mut input_event: EventWriter<InputEvent>,
    time: Res<Time>,
) {
    for gamepad in gamepads.iter() {
        button_inputs.get_pressed().for_each(|button| {
            match button {
                GamepadButton { gamepad: _, button_type } => {
                    match button_type {
                        GamepadButtonType::South => input_event.send(InputEvent::Jump),
                        GamepadButtonType::DPadUp => input_event.send(InputEvent::Zoom(10.0 * time.delta_seconds())),
                        GamepadButtonType::DPadDown => input_event.send(InputEvent::Zoom(-10.0 * time.delta_seconds())),
                        _ => {}
                    }
                }
        }});
        button_inputs.get_just_pressed().for_each(|button| {
            match button {
                GamepadButton { gamepad: _, button_type } => {
                    match button_type {
                        GamepadButtonType::East => input_event.send(InputEvent::ResetLevel),
                        GamepadButtonType::Start => input_event.send(InputEvent::Menu),
                        GamepadButtonType::Select => input_event.send(InputEvent::ToggleFullscreen),
                        _ => {}
                    }
                }
        }});

        let left_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
        if left_stick_x > 0.2 {
            input_event.send(InputEvent::MoveRight(left_stick_x));
        } else if left_stick_x < -0.2 {
            input_event.send(InputEvent::MoveLeft(-left_stick_x));
        }
    }
}