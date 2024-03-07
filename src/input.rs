use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;

use crate::state::ScheduleSet;

#[derive(Debug, Event)]
pub enum GameInputEvent {
    Zoom(f32),
    MoveLeft(f32),
    MoveRight(f32),
    Jump,
    ResetLevel,
    OpenMenu,
    ToggleFullscreen,
}

#[derive(Debug, Event)]
pub enum MenuInputEvent {
    Up,
    Down,
    Select,
    CloseMenu,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameInputEvent>()
            .add_event::<MenuInputEvent>()
            .add_systems(Update, (
                handle_keyboard_input,
                handle_mouse_wheel_input,
                gamepad_system,
            ).in_set(ScheduleSet::HandleInput)
        )
        .add_systems(Update, (
            handle_menu_event_keyboard,
            handle_menu_event_controler,
        ).in_set(ScheduleSet::PauseMenu));
    }
}

fn handle_menu_event_keyboard (
    mut menu_input_event: EventWriter<MenuInputEvent>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    keyboard_input.get_just_pressed().filter_map(|key| match key {
        KeyCode::Up => Some(MenuInputEvent::Up),
        KeyCode::Down => Some(MenuInputEvent::Down),
        KeyCode::Return => Some(MenuInputEvent::Select),
        KeyCode::Escape => Some(MenuInputEvent::CloseMenu),
        _ => None,
    }).for_each(|event| menu_input_event.send(event));
}

fn handle_menu_event_controler(
    button_inputs: Res<Input<GamepadButton>>,
    mut input_event: EventWriter<MenuInputEvent>,
) {
    button_inputs.get_just_pressed().for_each(|button| {
        match button {
            GamepadButton { gamepad: _, button_type } => {
                match button_type {
                    GamepadButtonType::DPadUp => input_event.send(MenuInputEvent::Up),
                    GamepadButtonType::DPadDown => input_event.send(MenuInputEvent::Down),
                    GamepadButtonType::South => input_event.send(MenuInputEvent::Select),
                    GamepadButtonType::Start => input_event.send(MenuInputEvent::CloseMenu),
                    _ => {}
                }
            }
    }});
    
}

fn handle_mouse_wheel_input(
    mut input_event: EventWriter<GameInputEvent>,
    mut ev_scroll: EventReader<MouseWheel>,
) {
    for ev in ev_scroll.read() {
        input_event.send(GameInputEvent::Zoom(ev.y));
    }
}

fn handle_keyboard_input(
    mut input_event: EventWriter<GameInputEvent>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    keyboard_input.get_pressed().filter_map(|key| match key {
        KeyCode::A => Some(GameInputEvent::MoveLeft(1.0)),
        KeyCode::D => Some(GameInputEvent::MoveRight(1.0)),
        KeyCode::Space => Some(GameInputEvent::Jump),
        _ => None,
    }).for_each(|event| input_event.send(event));
    keyboard_input.get_just_pressed().filter_map(|key| match key {
        KeyCode::R => Some(GameInputEvent::ResetLevel),
        KeyCode::Escape => Some(GameInputEvent::OpenMenu),
        KeyCode::F11 => Some(GameInputEvent::ToggleFullscreen),
        _ => None,
    }).for_each(|event| input_event.send(event));
}

fn gamepad_system(
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    mut input_event: EventWriter<GameInputEvent>,
    time: Res<Time>,
) {
    for gamepad in gamepads.iter() {
        button_inputs.get_pressed().for_each(|button| {
            match button {
                GamepadButton { gamepad: _, button_type } => {
                    match button_type {
                        GamepadButtonType::South => input_event.send(GameInputEvent::Jump),
                        GamepadButtonType::DPadUp => input_event.send(GameInputEvent::Zoom(10.0 * time.delta_seconds())),
                        GamepadButtonType::DPadDown => input_event.send(GameInputEvent::Zoom(-10.0 * time.delta_seconds())),
                        _ => {}
                    }
                }
        }});
        button_inputs.get_just_pressed().for_each(|button| {
            match button {
                GamepadButton { gamepad: _, button_type } => {
                    match button_type {
                        GamepadButtonType::East => input_event.send(GameInputEvent::ResetLevel),
                        GamepadButtonType::Start => input_event.send(GameInputEvent::OpenMenu),
                        GamepadButtonType::Select => input_event.send(GameInputEvent::ToggleFullscreen),
                        _ => {}
                    }
                }
        }});

        let left_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
        if left_stick_x > 0.2 {
            input_event.send(GameInputEvent::MoveRight(left_stick_x));
        } else if left_stick_x < -0.2 {
            input_event.send(GameInputEvent::MoveLeft(-left_stick_x));
        }
    }
}