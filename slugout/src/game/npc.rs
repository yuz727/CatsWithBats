/// Imports
use crate::game::npc_bully::*;
use crate::game::npc_events::*;
use crate::game::pathfinding::*;
use crate::GameState;
use crate::MAP;

use bevy::prelude::*;
use bevy::time::common_conditions::on_fixed_timer;
use bevy::time::common_conditions::on_timer;

use std::time::Duration;

use super::components::Ball;
use super::components::BallVelocity;
use super::components::Player;
use super::DIFFICULTY;

/// Constants for animation timer
const ANIM_TIME: f32 = 0.2;

/// Timer for swing interval
#[derive(Component, Deref, DerefMut)]
pub struct NPCTimer(Timer);

/// Timer for swing animation
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

/// Timer For Sidestep Duration
#[derive(Component, Deref, DerefMut)]
pub struct DangerTimer(Timer);

/// Velocity for NPC
#[derive(Component)]
pub struct NPCVelocity {
    pub velocity: Vec2,
}

/// NPC struct tag for entity
#[derive(Component)]
pub struct NPC;

/// NPC's Bat tag for entity
#[derive(Component)]
pub struct NPCBat;

/// A list of states that the NPC can be in
/// Each correspond to one type of movement
#[derive(Component)]
pub enum States {
    Default,
    Aggression,
    Evade,
    Idle,
    Danger,
}

/// The Map used for NPC pathfinding
/// One for storing coordinates, the other for walkable tiles
#[derive(Component)]
pub struct Maps {
    pub path_map: Vec<Vec<Vec2>>,
    pub walkable: Vec<Vec<bool>>,
}

/// The path generated by A* and the movement goal coordinates
#[derive(Component)]
pub struct Path {
    pub path: Vec<Vec2>,
    pub goal: Vec2,
    pub ball: Vec2,
}

/// Difficulty set to the NPC
#[derive(Component)]
pub struct Difficulty {
    pub difficulty: i32,
}

/// Update Path for NPC
impl Path {
    pub fn set_new_path(&mut self, new_path: Vec<Vec2>) {
        self.path = new_path;
    }
    pub fn set_new_ball(&mut self, new_ball_velocity: Vec2) {
        self.ball = new_ball_velocity;
    }
}

/// Struct Default initialisations
impl NPCVelocity {
    fn new() -> Self {
        Self {
            velocity: Vec2::splat(0.),
        }
    }
}

impl Path {
    fn new() -> Self {
        Self {
            path: Vec::with_capacity(1),
            goal: Vec2::splat(-1.),
            ball: Vec2::splat(-1.),
        }
    }
}

/// States Related Functions to check states
impl States {
    pub fn is_danger(&self) -> bool {
        match *self {
            States::Danger => true,
            _ => false,
        }
    }
    pub fn is_aggression(&self) -> bool {
        match *self {
            States::Aggression => true,
            _ => false,
        }
    }
    pub fn is_evade(&self) -> bool {
        match *self {
            States::Evade => true,
            _ => false,
        }
    }
}

/// Plugin for modular import
pub struct NPCPlugin {
    pub bully_mode: bool,
}

impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
        if self.bully_mode {
            // Easter egg Mode
            app.add_systems(
                Update,
                set_path
                    .run_if(in_state(GameState::Game))
                    .run_if(on_timer(Duration::from_secs(10))),
            );
            if unsafe { MAP == 1 } {
                app.add_systems(OnEnter(GameState::Game), load_npc_map1);
            } else if unsafe { MAP == 2 } {
                app.add_systems(OnEnter(GameState::Game), load_npc_map2);
            } else if unsafe { MAP == 3 } {
                app.add_systems(OnEnter(GameState::Game), load_npc_map3);
            } else if unsafe { MAP == 4 } {
                app.add_systems(OnEnter(GameState::Game), load_npc_map4);
            }
            app.add_systems(
                Update,
                approach_player.run_if(in_state(GameState::Game)), // Timer here to control the speed of the NPC
            );
            app.add_systems(Update, bully_swing.run_if(in_state(GameState::Game)));
        } else {
            // Spawn NPC with Different Attribute Depending on the map
            if unsafe { MAP == 1 } {
                app.add_systems(OnEnter(GameState::Game), load_npc_map1);
            } else if unsafe { MAP == 2 } {
                app.add_systems(OnEnter(GameState::Game), load_npc_map2);
            } else if unsafe { MAP == 3 } {
                app.add_systems(OnEnter(GameState::Game), load_npc_map3);
            } else if unsafe { MAP == 4 } {
                app.add_systems(OnEnter(GameState::Game), load_npc_map4);
            }
            app.add_systems(
                FixedUpdate,
                selection
                    .run_if(in_state(GameState::Game))
                    .run_if(on_fixed_timer(Duration::from_millis(200)))
                    .after(perform_a_star),
            );
            app.add_systems(
                Update,
                perform_a_star.run_if(in_state(GameState::Game)), //.run_if(on_fixed_timer(Duration::from_millis(17))),
            );
            app.add_systems(Update, swing.run_if(in_state(GameState::Game)));
            app.add_systems(Update, sidestep.run_if(in_state(GameState::Game)));
        }
    }
}

/// Load NPC entity and its bat
pub fn load_npc_map1(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn NPC Sprite
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Player4.png"),
            transform: Transform::with_scale(Transform::from_xyz(-320., 0., 3.), Vec3::splat(0.13)),
            ..default()
        })
        .insert(NPCTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .insert(NPC)
        .insert(NPCVelocity::new())
        .insert(States::Default)
        .insert(Difficulty {
            difficulty: unsafe { DIFFICULTY },
        })
        .insert(Maps {
            path_map: load_map_path(),
            walkable: load_walkable_map_1(),
        })
        .insert(Path::new())
        .insert(AnimationTimer(Timer::from_seconds(
            ANIM_TIME,
            TimerMode::Repeating,
        )))
        .insert(DangerTimer(Timer::from_seconds(1.0, TimerMode::Once)));

    //spawn bat sprite
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Bat.png"),
            transform: Transform::with_scale(Transform::from_xyz(-325., 0., 3.), Vec3::splat(0.13)),
            ..default()
        })
        .insert(NPCBat)
        .insert(NPCTimer(Timer::from_seconds(0.3, TimerMode::Repeating)))
        .insert(AnimationTimer(Timer::from_seconds(
            ANIM_TIME,
            TimerMode::Repeating,
        )));
}

pub fn load_npc_map2(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn NPC Sprite
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Player2.png"),
            transform: Transform::with_scale(Transform::from_xyz(-320., 0., 3.), Vec3::splat(0.13)),
            ..default()
        })
        .insert(NPCTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .insert(NPC)
        .insert(NPCVelocity::new())
        .insert(States::Default)
        .insert(Difficulty {
            difficulty: unsafe { DIFFICULTY },
        })
        .insert(Maps {
            path_map: load_map_path(),
            walkable: load_walkable_map_no_objects(),
        })
        .insert(Path::new())
        .insert(AnimationTimer(Timer::from_seconds(
            ANIM_TIME,
            TimerMode::Repeating,
        )))
        .insert(DangerTimer(Timer::from_seconds(1.0, TimerMode::Once)));

    //spawn bat sprite
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Bat.png"),
            transform: Transform::with_scale(Transform::from_xyz(-325., 0., 3.), Vec3::splat(0.13)),
            ..default()
        })
        .insert(NPCBat)
        .insert(NPCTimer(Timer::from_seconds(0.3, TimerMode::Repeating)))
        .insert(AnimationTimer(Timer::from_seconds(
            ANIM_TIME,
            TimerMode::Repeating,
        )));
}

pub fn load_npc_map3(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn NPC Sprite
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Player3.png"),
            transform: Transform::with_scale(Transform::from_xyz(-320., 0., 3.), Vec3::splat(0.13)),
            ..default()
        })
        .insert(NPCTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .insert(NPC)
        .insert(NPCVelocity::new())
        .insert(States::Default)
        .insert(Difficulty {
            difficulty: unsafe { DIFFICULTY },
        })
        .insert(Maps {
            path_map: load_map_path(),
            walkable: load_walkable_map_no_objects(),
        })
        .insert(Path::new())
        .insert(AnimationTimer(Timer::from_seconds(
            ANIM_TIME,
            TimerMode::Repeating,
        )))
        .insert(DangerTimer(Timer::from_seconds(1.0, TimerMode::Once)));

    //spawn bat sprite
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Bat.png"),
            transform: Transform::with_scale(Transform::from_xyz(-325., 0., 3.), Vec3::splat(0.13)),
            ..default()
        })
        .insert(NPCBat)
        .insert(NPCTimer(Timer::from_seconds(0.3, TimerMode::Repeating)))
        .insert(AnimationTimer(Timer::from_seconds(
            ANIM_TIME,
            TimerMode::Repeating,
        )));
}
pub fn load_npc_map4(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn NPC Sprite
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Player4.png"),
            transform: Transform::with_scale(Transform::from_xyz(-320., 0., 3.), Vec3::splat(0.13)),
            ..default()
        })
        .insert(NPCTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .insert(NPC)
        .insert(NPCVelocity::new())
        .insert(States::Default)
        .insert(Difficulty {
            difficulty: unsafe { DIFFICULTY },
        })
        .insert(Maps {
            path_map: load_map_path(),
            walkable: load_walkable_map_4(),
        })
        .insert(Path::new())
        .insert(AnimationTimer(Timer::from_seconds(
            ANIM_TIME,
            TimerMode::Repeating,
        )))
        .insert(DangerTimer(Timer::from_seconds(1.0, TimerMode::Once)));

    //spawn bat sprite
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Bat.png"),
            transform: Transform::with_scale(Transform::from_xyz(-325., 0., 3.), Vec3::splat(0.20)),
            ..default()
        })
        .insert(NPCBat)
        .insert(NPCTimer(Timer::from_seconds(0.3, TimerMode::Repeating)))
        .insert(AnimationTimer(Timer::from_seconds(
            ANIM_TIME,
            TimerMode::Repeating,
        )));
}

/// Behaviour Tree Logic & Action Selector
/// For the graph below: Go to next line for failed checks
/// For a more detailed version, check the spec sheet
/// The acutal movement will follow the update ticks, the logic below is simply for Target setting
/// Swinging will follow by the target check (Whether it is close to a target)
//  Root -> Danger Check -> Difficulty Check -> Set goal to closest ball & Aggression Mode
//                      -> Sidestep & Danger Mode
//      -> Player Close Check -> Difficulty Check -> Set goal to player & Aggression Mode
//      -> Difficulty Check -> Set goal to random ball & Aggression Mode
//      -> Evade Check -> Set goal to behind the closest object
//                     -> Idle
pub fn selection(
    mut npc: Query<
        (
            &mut Transform,
            &mut NPCVelocity,
            &mut Path,
            &Maps,
            &Difficulty,
            &mut States,
        ),
        (With<NPC>, Without<NPCBat>, Without<Ball>, Without<Player>),
    >,
    player: Query<&mut Transform, (With<Player>, Without<NPC>, Without<Ball>, Without<NPCBat>)>,
    time: Res<Time>,
    ball_query: Query<
        (&Transform, &BallVelocity, &Ball),
        (With<Ball>, Without<NPC>, Without<NPCBat>, Without<Player>),
    >,
) {
    for (npc_transform, _velocity, mut path, maps, difficulty, mut state) in npc.iter_mut() {
        for player_transform in player.iter() {
            let danger = danger_check(npc_transform.translation, &time, &ball_query);
            if danger {
                if difficulty_check(difficulty.difficulty) {
                    if set_tag_to_closest_ball(npc_transform.translation, &mut path, &ball_query) {
                        set_a_star(npc_transform.translation, &mut path, maps);
                        *state = States::Aggression;
                        // info!("Aggression");
                        return;
                    } // if
                } // if
                *state = States::Danger;
                // info!("Danger");
                return;
            } // if danger
            if tag_is_null(&path)
                && player_proximity_check(npc_transform.translation, player_transform.translation)
                && difficulty_check(difficulty.difficulty)
            {
                set_tag_to_player(&mut path, player_transform.translation);
                set_a_star(npc_transform.translation, &mut path, maps);
                *state = States::Aggression;
                // info!("Aggression");
                return;
            } // if player close
            if difficulty_check(difficulty.difficulty) {
                if set_tag_to_closest_ball(npc_transform.translation, &mut path, &ball_query) {
                    set_a_star(npc_transform.translation, &mut path, maps);
                    *state = States::Aggression;
                    // info!("Aggression");
                    return;
                } // if
            } // if
            if difficulty_check(difficulty.difficulty) {
                // Evasion check, Chance to evade = difficulty as percent
                if set_tag_to_closest_object(npc_transform.translation, &mut path) {
                    set_a_star(npc_transform.translation, &mut path, maps);
                    *state = States::Evade;
                    // info!("Evade");
                    return;
                } // if
            } // if
            *state = States::Idle;
            // info!("Idle");
            return;
        } // for
    } // for
}
