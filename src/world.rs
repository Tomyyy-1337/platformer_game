use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::{control::KinematicCharacterController, dynamics::RigidBody, geometry::{Collider, Friction}};
use std::collections::{HashMap, HashSet};
use crate::player;
use crate::state;
use crate::input;
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            LdtkPlugin,
        ))
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .register_ldtk_int_cell::<WallBundle>(1)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .add_systems(Startup, (
            setup,
        ))
        .add_systems(Update, (
            spawn_wall_collision,
            update_level_selection,
            restart_level,
        ).in_set(state::ScheduleSet::MainUpdate));
    }
            
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ldtk_handle = asset_server.load("tile-based-game.ldtk");
    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Clone, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_bundle("player.png")]
    pub sprite_bundle: SpriteBundle,
    pub player: player::Player,
    #[from_entity_instance]
    entity_instance: EntityInstance,
    velocity: player::Velocity,
    controller: KinematicCharacterController,
    #[from_entity_instance]
    collider: ColliderBundle,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            sprite_bundle: SpriteBundle {
                ..Default::default()
            },
            player: player::Player,
            entity_instance: Default::default(),
            velocity: Default::default(),
            controller: KinematicCharacterController {
                autostep: None,
                snap_to_ground: None,
                custom_mass: Some(100.0),
                ..Default::default()
            },
            collider: Default::default(),
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(entity_instance: &EntityInstance) -> ColliderBundle {

        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: Collider::cuboid(14.0, 14.0),
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}

pub fn spawn_wall_collision(
    mut commands: Commands,
    wall_query: Query<(&GridCoords, &Parent), Added<Wall>>,
    parent_query: Query<&Parent, Without<Wall>>,
    level_query: Query<(Entity, &LevelIid)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut state: ResMut<NextState<state::AppState>>,
) {
    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    let mut level_to_wall_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    wall_query.for_each(|(&grid_coords, parent)| {
        if let Ok(grandparent) = parent_query.get(parent.get()) {
            level_to_wall_locations
                .entry(grandparent.get())
                .or_default()
                .insert(grid_coords);
        }
    });

    if !wall_query.is_empty() {
        level_query.for_each(|(level_entity, level_iid)| {
            if let Some(level_walls) = level_to_wall_locations.get(&level_entity) {
                let ldtk_project = ldtk_project_assets
                    .get(ldtk_projects.single())
                    .expect("Project should be loaded if level has spawned");

                let level = ldtk_project
                    .as_standalone()
                    .get_loaded_level_by_iid(&level_iid.to_string())
                    .expect("Spawned level should exist in LDtk project");

                let LayerInstance {
                    c_wid: width,
                    c_hei: height,
                    grid_size,
                    ..
                } = level.layer_instances()[0];

                let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

                for y in 0..height {
                    let mut row_plates: Vec<Plate> = Vec::new();
                    let mut plate_start = None;

                    for x in 0..width + 1 {
                        match (plate_start, level_walls.contains(&GridCoords { x, y })) {
                            (Some(s), false) => {
                                row_plates.push(Plate {
                                    left: s,
                                    right: x - 1,
                                });
                                plate_start = None;
                            }
                            (None, true) => plate_start = Some(x),
                            _ => (),
                        }
                    }

                    plate_stack.push(row_plates);
                }

                let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
                let mut prev_row: Vec<Plate> = Vec::new();
                let mut wall_rects: Vec<Rect> = Vec::new();

                plate_stack.push(Vec::new());

                for (y, current_row) in plate_stack.into_iter().enumerate() {
                    for prev_plate in &prev_row {
                        if !current_row.contains(prev_plate) {
                            // remove the finished rect so that the same plate in the future starts a new rect
                            if let Some(rect) = rect_builder.remove(prev_plate) {
                                wall_rects.push(rect);
                            }
                        }
                    }
                    for plate in &current_row {
                        rect_builder
                            .entry(plate.clone())
                            .and_modify(|e| e.top += 1)
                            .or_insert(Rect {
                                bottom: y as i32,
                                top: y as i32,
                                left: plate.left,
                                right: plate.right,
                            });
                    }
                    prev_row = current_row;
                }

                commands.entity(level_entity).with_children(|level| {
                    for wall_rect in wall_rects {
                        level
                            .spawn_empty()
                            .insert(Collider::cuboid(
                                (wall_rect.right as f32 - wall_rect.left as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                                (wall_rect.top as f32 - wall_rect.bottom as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                            ))
                            .insert(RigidBody::Fixed)
                            .insert(Friction::new(1.0))
                            .insert(Transform::from_xyz(
                                (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32
                                    / 2.,
                                (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32
                                    / 2.,
                                0.,
                            ))
                            .insert(GlobalTransform::default());
                    }
                });
            }
        });
    }
    state.set(state::AppState::Running);
}

pub fn update_level_selection(
    level_query: Query<(&LevelIid, &Transform), Without<player::Player>>,
    player_query: Query<&Transform, With<player::Player>>,
    mut level_selection: ResMut<LevelSelection>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    for (level_iid, level_transform) in &level_query {
        let ldtk_project = ldtk_project_assets
            .get(ldtk_projects.single())
            .expect("Project should be loaded if level has spawned");

        let level = ldtk_project
            .get_raw_level_by_iid(&level_iid.to_string())
            .expect("Spawned level should exist in LDtk project");

        let level_bounds = Rect {
            min: Vec2::new(level_transform.translation.x, level_transform.translation.y),
            max: Vec2::new(
                level_transform.translation.x + level.px_wid as f32,
                level_transform.translation.y + level.px_hei as f32,
            ),
        };

        for player_transform in &player_query {
            if player_transform.translation.x < level_bounds.max.x
                && player_transform.translation.x > level_bounds.min.x
                && player_transform.translation.y < level_bounds.max.y
                && player_transform.translation.y > level_bounds.min.y
                && !level_selection.is_match(&LevelIndices::default(), level)
            {
                *level_selection = LevelSelection::iid(level.iid.clone());
            }
        }
    }
}

pub fn restart_level(
    mut commands: Commands,
    level_query: Query<Entity, With<LevelIid>>,
    mut input: EventReader<input::InputEvent>
) {
    for event in input.read() {
        match event {
            input::InputEvent::ResetLevel => {
                for level_entity in &level_query {
                    commands.entity(level_entity).insert(Respawn);
                }
            }
            _ => {}
        }
    }

}