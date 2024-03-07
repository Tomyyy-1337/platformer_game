use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::player;
use crate::state::ScheduleSet;

#[derive(Component)]
pub struct PlayerSprite;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum LoadState {
    #[default]
    AssetLoading,
    Done,
}

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<LoadState>()
            .add_loading_state(
                LoadingState::new(LoadState::AssetLoading)
                    .continue_to_state(LoadState::Done)
                    .load_collection::<PlayerAssets>(),
            )
            .add_systems(OnEnter(LoadState::Done), (spawn_player_sprite,))
            .add_systems(
                Update,
                (animate_sprite_system, update_player_sprite_pos)
                    .run_if(in_state(LoadState::Done))
                    .in_set(ScheduleSet::PostTransformUpdate),
            );
    }
}

fn update_player_sprite_pos(
    player_query: Query<(&Transform, With<player::Player>)>,
    mut player_sprite_query: Query<(
        &mut Transform,
        (With<PlayerSprite>, Without<player::Player>),
    )>,
) {
    for player_transform in &mut player_query.iter() {
        for mut sprite_transform in &mut player_sprite_query.iter_mut() {
            sprite_transform.0.translation =
                player_transform.0.translation + Vec3::new(-700.0, -515.0, 5.0);
        }
    }
}

fn spawn_player_sprite(mut commands: Commands, my_assets: Res<PlayerAssets>) {
    println!("Spawning player sprite");
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
    ));
}

#[derive(Component)]
struct AnimationTimer(Timer);

fn animate_sprite_system(
    time: Res<Time>,
    mut sprites_to_animate: Query<(&mut AnimationTimer, &mut TextureAtlasSprite)>,
) {
    for (mut timer, mut sprite) in &mut sprites_to_animate {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index = (sprite.index + 1) % 8;
        }
    }
}

#[derive(AssetCollection, Resource)]
struct PlayerAssets {
    #[asset(texture_atlas(tile_size_x = 128., tile_size_y = 128., columns = 8, rows = 1))]
    #[asset(image(sampler = nearest))]
    #[asset(path = "textures/Fighter/Walk.png")]
    player_walk: Handle<TextureAtlas>,
}
