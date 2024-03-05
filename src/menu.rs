use bevy::{prelude::*, window::PresentMode};
use bevy::window::{CursorGrabMode, PrimaryWindow};
use crate::asset_loader::FontAssets;
use crate::state::ScheduleSet;
use crate::state::AppState;

#[derive(Component)]
pub struct FortsetzenButton;

#[derive(Component)]
pub struct QuitButton;

#[derive(Component)]
pub struct FramerateButton;

/// Marker component for menu items.
#[derive(Component)]
pub struct MenuItem;


/// Plugin for the menu system.
pub struct MenuPlugin;


/// Implementation of the `Plugin` trait for the `MenuPlugin` struct.
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
            app.add_systems(Update, (
                toggle_menu,
                clear_menu,
                toggle_cursor_visibiltiy,
                toggle_framerate,
            ).in_set(ScheduleSet::CheckMenu))
            .add_systems(Update, (
                fortsetzen_button,
                quit_game_button,
                button_hower,
            ).in_set(ScheduleSet::PauseMenu));
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
            primary_window.cursor.grab_mode = CursorGrabMode::None;
            primary_window.cursor.visible = true;
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
    keyboard_inputs: Res<Input<KeyCode>>,
    simulation_state: Res<State<AppState>>,
    font_assets: Res<FontAssets>,    
) {
    if keyboard_inputs.just_pressed(KeyCode::Escape) {
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

                                // Toggle Hitboxes Button
                                parent.spawn((
                                    FramerateButton,
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
            AppState::Menu => {
                commands.insert_resource(NextState(Some(AppState::Running)));
            },
        };
    }
}

fn button_hower(
    mut interaction_query: Query<(&Interaction,&mut BackgroundColor),(Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut background_color) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Hovered => {
                background_color.0 = Color::rgba(0.0, 0.0, 0.0, 0.8);
            },
            Interaction::None => {
                background_color.0 = Color::rgba(0.0, 0.0, 0.0, 0.5);
            },
            _ => {},
        }
    }
}

fn fortsetzen_button(
    mut interaction_query: Query<&Interaction,(Changed<Interaction>, With<FortsetzenButton>)>,
    mut commands: Commands,
) {
    for interaction in interaction_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                commands.insert_resource(NextState(Some(AppState::Running)));
            },
            _ => {},
        }
    }
}

fn quit_game_button(
    mut interaction_query: Query<&Interaction,(Changed<Interaction>, With<QuitButton>)>,
) {
    for interaction in interaction_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                std::process::exit(0);
            },
            _ => {},
        }
    }
}

pub fn toggle_framerate(
    mut interaction_query: Query<&Interaction,(Changed<Interaction>, With<FramerateButton>)>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    for interaction in interaction_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                for mut w in window_query.iter_mut() {
                    w.present_mode = match w.present_mode {
                        PresentMode::Mailbox => PresentMode::Fifo,
                        PresentMode::Fifo => PresentMode::Mailbox,
                        _ => PresentMode::Fifo,
                    };
                }
            },
            _ => {},
        }
    }
}