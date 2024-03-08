use std::thread::current;
use std::time::Duration;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::state::ScheduleSet;
use crate::player;

#[derive(Component)]
pub struct PlayerSprite;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum LoadState {
    #[default]
    AssetLoading,
    Done,
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Copy)]
pub enum PlayerAnimationType {
    Idle,
    Walk,
    Run,
    Jump,
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Copy)]
pub enum AnimationDirection {
    Left,
    Right,
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Copy)]
pub struct PlayerAnimation {
    pub current_animation: PlayerAnimationType,
    pub current_direction: AnimationDirection,
}

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<LoadState>()
            .add_loading_state(
                LoadingState::new(LoadState::AssetLoading)
                    .continue_to_state(LoadState::Done)
                    .load_collection::<PlayerAssets>()
            )
            .add_systems(
                OnEnter(LoadState::Done),
                (
                    spawn_player_sprite,
                ),
            )
            .add_systems(
                Update,
                (
                    animate_player_sprite_system,
                    update_player_sprite_pos,
                    update_player_animation,
                ).run_if(in_state(LoadState::Done)).in_set(ScheduleSet::PostTransformUpdate),
            );
    }
}

fn update_player_sprite_pos(
    player_query: Query<(&Transform, With<player::Player>)>,
    mut player_sprite_query: Query<(&mut Transform, (With<PlayerSprite>, Without<player::Player>))>,
) {
    for player_transform in &mut player_query.iter() {
        for mut sprite_transform in &mut player_sprite_query.iter_mut() {
            sprite_transform.0.translation = player_transform.0.translation + Vec3::new(-705.0, -515.0, 5.0);
        }
    }
}

fn spawn_player_sprite (
    mut commands: Commands,
    my_assets: Res<PlayerAssets>,
) {
    commands.spawn((
        PlayerSprite,
        SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(0., 150., 3.0),
                scale: Vec3::new(0.5, 0.5, 1.0),
                ..Default::default()
            },
            sprite: TextureAtlasSprite {
                index: 0,
                // flip_x: true,
                ..Default::default()
            },
            texture_atlas: my_assets.player_walk.clone(),
            ..Default::default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        PlayerAnimation {
            current_animation: PlayerAnimationType::Idle,
            current_direction: AnimationDirection::Left,
        },
    ));
    
}

fn update_player_animation(
    player_query: Query<&player::Velocity, With<player::Player>>,
    mut player_sprite_query: Query<(&mut PlayerAnimation, &mut TextureAtlasSprite), With<PlayerSprite>>,
) {
    for velocity in &mut player_query.iter() {
        for (mut animation,mut sprite) in &mut player_sprite_query.iter_mut() {
            if velocity.0.x > 1.0 {
                animation.current_direction = AnimationDirection::Right;
            } else if velocity.0.x < -1.0 {
                animation.current_direction = AnimationDirection::Left;
            };
            if animation.current_animation == PlayerAnimationType::Jump {
                if sprite.index == 29 {
                    animation.current_animation = PlayerAnimationType::Idle;
                    sprite.index = 30;
                } else {
                    continue;
                }
            } 
            if velocity.0.y > 100.0 {
                if animation.current_animation != PlayerAnimationType::Jump {
                    sprite.index = 20;
                }
                animation.current_animation = PlayerAnimationType::Jump;
            } else if velocity.0.x.abs() > 100.0 {
                if animation.current_animation != PlayerAnimationType::Run {
                    sprite.index = 0;
                }
                animation.current_animation = PlayerAnimationType::Run;
            } else if velocity.0.x.abs() > 5.0 {
                if animation.current_animation != PlayerAnimationType::Walk {
                    sprite.index = 0;
                }
                animation.current_animation = PlayerAnimationType::Walk;
            } else {
                if animation.current_animation != PlayerAnimationType::Idle {
                    sprite.index = 30;
                }
                animation.current_animation = PlayerAnimationType::Idle;
            }
        }
    }
}

#[derive(Component)]
struct AnimationTimer(Timer);

fn animate_player_sprite_system(
    time: Res<Time>,
    mut sprites_to_animate: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &PlayerAnimation), With<PlayerSprite>>,
) {
    for (mut timer, mut sprite, player_animation) in &mut sprites_to_animate {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            match player_animation.current_animation {
                PlayerAnimationType::Idle => {
                    sprite.index = 30 + ((sprite.index + 1) % 6); 
                }
                PlayerAnimationType::Walk => {
                    sprite.index = (sprite.index + 1) % 8;
                }
                PlayerAnimationType::Run => {
                    sprite.index = 10 + (sprite.index + 1) % 8;
                }
                PlayerAnimationType::Jump => {
                    sprite.index = 20 + (sprite.index + 1) % 10;
                }
            }
        }
        match player_animation.current_direction {
            AnimationDirection::Left => {
                sprite.flip_x = true;
            }
            AnimationDirection::Right => {
                sprite.flip_x = false;
            }
        }
    }
}

#[derive(AssetCollection, Resource)]
struct PlayerAssets {
    #[asset(texture_atlas(tile_size_x = 128., tile_size_y = 128., columns = 10, rows = 4))]
    #[asset(image(sampler = nearest))]
    #[asset(path = "textures/Fighter/player.png")]
    player_walk: Handle<TextureAtlas>,
}