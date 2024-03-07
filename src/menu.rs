use bevy::ecs::event;
use bevy::{prelude::*, window::PresentMode};
use bevy::window::{CursorGrabMode, PrimaryWindow};
use crate::asset_loader::FontAssets;
use crate::input::{GameInputEvent, MenuInputEvent};
use crate::state::ScheduleSet;
use crate::state::AppState;

#[derive(Component)]
pub struct FortsetzenButton;

#[derive(Component)]
pub struct FullscreenButton;

#[derive(Component)]
pub struct QuitButton;

#[derive(Component)]
pub struct FramerateButton;

#[derive(Component)]
pub struct MenuItem;

pub struct MenuPlugin;

#[derive(Component, Clone, Copy)]
pub struct Identifier(ButtonType);


#[derive(States, Clone, Copy, Default, Debug, Hash, PartialEq, Eq)]
enum ButtonType {
    #[default]
    Fortsetzen,
    Fullscreen,
    Framerate,
    Quit,
}

impl ButtonType {
    fn next(&self) -> ButtonType{
        match self {
            ButtonType::Fortsetzen => ButtonType::Fullscreen,
            ButtonType::Fullscreen => ButtonType::Framerate,
            ButtonType::Framerate => ButtonType::Quit,
            ButtonType::Quit => ButtonType::Fortsetzen,
        }
    }

    fn previous(&self) -> ButtonType{
        match self {
            ButtonType::Fortsetzen => ButtonType::Quit,
            ButtonType::Fullscreen => ButtonType::Fortsetzen,
            ButtonType::Framerate => ButtonType::Fullscreen,
            ButtonType::Quit => ButtonType::Framerate,
        }
    }
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
            app.add_state::<ButtonType>()
                .add_systems(Update, (
                    toggle_menu,
                    clear_menu,
                    toggle_cursor_visibiltiy,
                    toggle_fullscreen_key,
                ).in_set(ScheduleSet::CheckMenu))
                .add_systems(Update, (
                    toggle_fullscreen_ui,
                    toggle_framerate_button,
                    fortsetzen_button,
                    quit_game_button,
                    button_hower,
                    set_background_color,
                    handle_menu_event,
                ).in_set(ScheduleSet::PauseMenu));
    }
        
}

fn handle_menu_event(
    mut menu_input_event: EventReader<MenuInputEvent>,
    mut commands: Commands,
    active_button: ResMut<State<ButtonType>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    for ev in menu_input_event.read() {
        match ev {
            MenuInputEvent::Up => commands.insert_resource(NextState(Some(active_button.get().previous()))),
            MenuInputEvent::Down => commands.insert_resource(NextState(Some(active_button.get().next()))),
            MenuInputEvent::Select => {
                match active_button.get() {
                    ButtonType::Fortsetzen => fortsetzten_action(&mut commands),
                    ButtonType::Quit => quit_game_action(),
                    ButtonType::Framerate => {
                        toggle_framerate_action(&mut window_query);
                        return;
                    
                    },
                    ButtonType::Fullscreen => {
                        toggle_fullscreen(window_query);
                        return;
                    },
                    
                }
            }
            MenuInputEvent::CloseMenu => fortsetzten_action(&mut commands),
        }
    }
}

/// System to toggle the visibility of the cursor when the menu state changes.
fn toggle_cursor_visibiltiy(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    simulation_state: Res<State<AppState>>,
) {
    match simulation_state.get() {
        AppState::Menu if simulation_state.is_changed() => {
            let mut primary_window = q_windows.single_mut();
            let width = primary_window.width();
            let height = primary_window.height();
            primary_window.cursor.grab_mode = CursorGrabMode::None;
            primary_window.cursor.visible = true;
            primary_window.set_cursor_position(Some(Vec2::new(width / 2.0, height / 2.0 )));
        },
        AppState::Running if simulation_state.is_changed() => {
            let mut primary_window = q_windows.single_mut();
            primary_window.cursor.grab_mode = CursorGrabMode::Locked;
            primary_window.cursor.visible = false;
        },
        _ => {},
    }
}

/// System to clear the menu when the menu state changes to `AppState::Menu`.
fn clear_menu(
    mut commands: Commands,
    simulation_state: Res<State<AppState>>,
    mut query: Query<(Entity, &MenuItem)>,
) {
    match simulation_state.get() {
        AppState::Running if simulation_state.is_changed() => {
            for (entity, _) in query.iter_mut() {
                commands.entity(entity).despawn_recursive();
            }
        },
        _ => {},
    }
}

/// System to toggle the menu state when the Escape key is pressed.
fn toggle_menu(
    mut commands: Commands,
    mut ingame_events: EventReader<GameInputEvent>,
    mut menu_events: EventReader<MenuInputEvent>,
    simulation_state: Res<State<AppState>>,
    font_assets: Res<FontAssets>,    
) {
    for event in menu_events.read() {
        match event {
            MenuInputEvent::CloseMenu => {
                commands.insert_resource(NextState(Some(AppState::Running)));
            },
            _ => (),
        }
    }

    for event in ingame_events.read() {
        match event {
            GameInputEvent::OpenMenu => {
                match simulation_state.get() {
                    AppState::Running => {
                        commands.insert_resource(NextState(Some(AppState::Menu)));
                        commands.spawn((
                            MenuItem,
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                ..default()
                            },
                            ))
                            .with_children(|parent| {
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::Column,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn(TextBundle::from_section(
                                            "Test Game",
                                            TextStyle {
                                                font: font_assets.menu_font.clone(),
                                                font_size: 80.0,
                                                color: Color::RED,
                                                },
                                            ).with_style(Style {
                                                margin: UiRect::all(Val::Px(30.0)),
                                                ..default()
                                            }),
                                        );
                                        parent.spawn(TextBundle::from_section(
                                            "Spiel Pausiert",
                                            TextStyle {
                                                font: font_assets.menu_font.clone(),
                                                font_size: 50.0,
                                                color: Color::WHITE,
                                                },
                                            ).with_style(Style {
                                                margin: UiRect::all(Val::Px(15.0)),
                                                ..default()
                                            }),
                                        );
                                        parent.spawn(TextBundle::from_section(
                                            "Escape zum Fortsetzen",
                                            TextStyle {
                                                font: font_assets.menu_font.clone(),
                                                font_size: 50.0,
                                                color: Color::WHITE,
                                                },
                                            ).with_style(Style {
                                                margin: UiRect::all(Val::Px(15.0)),
                                                ..default()
                                            }),
                                        );
        
                                        // Fortsetzen Button
                                        parent.spawn((
                                            FortsetzenButton,
                                            Identifier(ButtonType::Fortsetzen),
                                            ButtonBundle {
                                                style: Style {
                                                    margin: UiRect::all(Val::Px(15.0)),
                                                    ..default()
                                                },
                                                background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
                                                ..Default::default()
                                        }))
                                        .with_children(|parent| {
                                            parent.spawn((MenuItem, TextBundle::from_section(
                                                "Fortsetzen",
                                                TextStyle {
                                                    font: font_assets.menu_font.clone(),
                                                    font_size: 25.0,
                                                    color: Color::WHITE,
                                                },
                                            ).with_style(
                                                Style {
                                                    margin: UiRect::all(Val::Px(15.0)),
                                                    ..default()
                                                }
                                            )));
                                        });
        
                                        // Toggle Fullscreen Button
                                        parent.spawn((
                                            FullscreenButton,
                                            Identifier(ButtonType::Fullscreen),
                                            ButtonBundle {
                                                style: Style {
                                                    margin: UiRect::all(Val::Px(15.0)),
                                                    ..default()
                                                },
                                                background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
                                                ..Default::default()
                                        }))
                                        .with_children(|parent| {
                                            parent.spawn((MenuItem, TextBundle::from_section(
                                                "Toggle Fullscreen",
                                                TextStyle {
                                                    font: font_assets.menu_font.clone(),
                                                    font_size: 25.0,
                                                    color: Color::WHITE,
                                                },
                                            ).with_style(
                                                Style {
                                                    margin: UiRect::all(Val::Px(15.0)),
                                                    ..default()
                                                }
                                            )));
                                        });
        
                                        // Toggle V-Sync
                                        parent.spawn((
                                            FramerateButton,
                                            Identifier(ButtonType::Framerate),
                                            ButtonBundle {
                                                style: Style {
                                                    margin: UiRect::all(Val::Px(15.0)),
                                                    ..default()
                                                },
                                                background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
                                                ..Default::default()
                                        }))
                                        .with_children(|parent| {
                                            parent.spawn((MenuItem, TextBundle::from_section(
                                                "Toggle Vsync",
                                                TextStyle {
                                                    font: font_assets.menu_font.clone(),
                                                    font_size: 25.0,
                                                    color: Color::WHITE,
                                                },
                                            ).with_style(
                                                Style {
                                                    margin: UiRect::all(Val::Px(15.0)),
                                                    ..default()
                                                }
                                            )));
                                        });
        
                                        // Quit Button
                                        parent.spawn((
                                            QuitButton,
                                            Identifier(ButtonType::Quit),
                                            ButtonBundle {
                                                style: Style {
                                                    margin: UiRect::all(Val::Px(15.0)),
                                                    ..default()
                                                },
                                                background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
                                                ..Default::default()
                                        }))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                QuitButton,
                                                TextBundle::from_section(
                                                "Spiel Beenden",
                                                TextStyle {
                                                    font: font_assets.menu_font.clone(),
                                                    font_size: 25.0,
                                                    color: Color::WHITE,
                                                },
                                            ).with_style(
                                                Style {
                                                    margin: UiRect::all(Val::Px(15.0)),
                                                    ..default()
                                                }
                                            )));
                                        });
                                });
                        });
                    },
                    _ => (),
                };
            },
            _ => ()
        }
    }
}

fn button_hower(
    mut interaction_query: Query<(&Interaction, &Identifier),(Changed<Interaction>, With<Button>)>,
    mut commands: Commands,
) {
    for (interaction, identifier) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Hovered => {
                commands.insert_resource(NextState(Some(identifier.0)));
            },
            _ => {},
        }
    }
}

fn set_background_color(
    mut background_color_query: Query<(&mut BackgroundColor, &Identifier),With<Button>>,
    active_button: Res<State<ButtonType>>,
) {
    for (mut background_color, identifier) in background_color_query.iter_mut() {
        background_color.0 = if identifier.0 == *active_button.get() {
            Color::rgba(0.0, 0.0, 0.0, 0.8)
        } else {
            Color::rgba(0.0, 0.0, 0.0, 0.5)
        };

    }
}

fn fortsetzen_button(
    mut interaction_query: Query<&Interaction,(Changed<Interaction>, With<FortsetzenButton>)>,
    mut commands: Commands,
) {
    for interaction in interaction_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                fortsetzten_action(&mut commands);
            },
            _ => {},
        }
    }
}

fn fortsetzten_action(commands: &mut Commands<'_, '_>) {
    commands.insert_resource(NextState(Some(AppState::Running)));
}


fn quit_game_button(
    mut interaction_query: Query<&Interaction,(Changed<Interaction>, With<QuitButton>)>,
) {
    for interaction in interaction_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                quit_game_action();
            },
            _ => {},
        }
    }
}

fn quit_game_action() {
    std::process::exit(0);
}

pub fn toggle_framerate_button(
    mut interaction_query: Query<&Interaction,(Changed<Interaction>, With<FramerateButton>)>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    for interaction in interaction_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                toggle_framerate_action(&mut window_query);
            },
            _ => {},
        }
    }
}

fn toggle_framerate_action(window_query: &mut Query<'_, '_, &mut Window, With<PrimaryWindow>>) {
    for mut w in window_query.iter_mut() {
        w.present_mode = match w.present_mode {
            PresentMode::Mailbox => PresentMode::Fifo,
            PresentMode::Fifo => PresentMode::Mailbox,
            _ => PresentMode::Fifo,
        };
    }
}

fn toggle_fullscreen_key(
    mut keyboard_inputs: EventReader<GameInputEvent>,
    window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    for event in keyboard_inputs.read() {
        match event {
            GameInputEvent::ToggleFullscreen => {
                toggle_fullscreen(window_query);
                return;
            },
            _ => {},
        }
    }
}

fn toggle_fullscreen_ui(
    window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut interaction_query: Query<&Interaction,(Changed<Interaction>, With<FullscreenButton>)>,
) {
    for interaction in interaction_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                toggle_fullscreen(window_query);
                return;
            },
            _ => {},
        }
    }
}

fn toggle_fullscreen(mut window_query: Query<'_, '_, &mut Window, With<PrimaryWindow>>) {
    for mut w in window_query.iter_mut() {
        w.mode = match w.mode {
            bevy::window::WindowMode::Windowed => bevy::window::WindowMode::BorderlessFullscreen,
            _ => bevy::window::WindowMode::Windowed,
        };
    }
}