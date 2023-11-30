// use crate::components::*;

use crate::game::npc_events::*;
// use crate::game::npc_tree::Node;
// use crate::game::npc_tree::*;
use crate::game::pathfinding::*;
use crate::GameState;
use bevy::prelude::*;
use bevy::utils::synccell::SyncCell;
//use bevy::time::Stopwatch;
use rand::prelude::*;

use super::components::Ball;
use super::components::Player;

const ANIM_TIME: f32 = 0.2;

// Timer for movement
#[derive(Component, Deref, DerefMut)]
pub struct NPCTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct NPCVelocity {
    pub velocity: Vec2,
    pub xlock: i32,
    pub ylock: i32,
}

#[derive(Component)]
pub struct NPC;

#[derive(Component)]
pub struct NPCBat;

#[derive(Component)]
pub struct NPCFace;

#[derive(Component)]
pub enum States {
    Aggression,
    Evade,
    Idle,
}

// #[derive(Component)]
// pub struct Tree {
//     root_node: Node,
// }

#[derive(Component)]
pub struct Maps {
    pub path_map: Vec<Vec<Vec2>>,
    pub walkable: Vec<Vec<bool>>,
}

#[derive(Component)]
pub struct Path {
    pub path: Vec<Vec2>,
    pub goal: Vec2,
}

#[derive(Component)]
pub struct Difficulty {
    pub difficulty: i32,
}

impl Path {
    pub fn set_new_path(&mut self, new_path: Vec<Vec2>) {
        self.path = new_path;
    }
}

impl States {
    fn to_aggression(&mut self) {
        *self = match std::mem::replace(self, States::Aggression) {
            States::Idle => States::Aggression,
            States::Evade => States::Aggression,
            v => v,
        }
    }
    fn to_evade(&mut self) {
        *self = match std::mem::replace(self, States::Evade) {
            States::Idle => States::Evade,
            States::Aggression => States::Evade,
            v => v,
        }
    }
    // fn to_aggression_ball(&mut self) {
    //     *self = match std::mem::replace(self, States::AggressionBall) {
    //         States::Aggression => States::AggressionBall,
    //         v => v,
    //     }
    // }
    // fn to_aggression_player(&mut self) {
    //     *self = match std::mem::replace(self, States::AggressionPlayer) {
    //         States::Aggression => States::AggressionPlayer,
    //         v => v,
    //     }
    // }
    fn to_idle(&mut self) {
        *self = match std::mem::replace(self, States::Idle) {
            States::Aggression => States::Idle,
            // States::AggressionBall => States::Idle,
            // States::AggressionPlayer => States::Idle,
            States::Evade => States::Idle,
            v => v,
        }
    }
}

impl NPCVelocity {
    fn new() -> Self {
        Self {
            velocity: Vec2::splat(0.),
            xlock: 0,
            ylock: 0,
        }
    }
    pub fn lock_x(&mut self) {
        self.xlock = 1;
    }
    pub fn lock_y(&mut self) {
        self.ylock = 1;
    }
    pub fn unlock_x(&mut self) {
        self.xlock = 0;
    }
    pub fn unlock_y(&mut self) {
        self.ylock = 0;
    }
}

pub struct NPCPlugin;

impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), load_npc);
        // app.add_systems(OnEnter(GameState::Game), behavior_tree);
        // app.add_systems(Update, execute_node.after(load_npc).after(behavior_tree));
        //app.add_systems(Update, select_bully_mode.run_if(in_state(GameState::Game)));
        //app.add_systems(OnEnter(GameState::Game), load_map);
        // app.add_systems(Update, select.run_if(in_state(GameState::Game)));
        // app.add_systems(Update, avoid_collision.run_if(in_state(GameState::Game)));
        // app.add_systems(
        //     Update,
        //     approach_player
        //         .after(select)
        //         .run_if(in_state(GameState::Game)),
        // );
        // app.add_systems(
        //     Update,
        //     approach_ball
        //         .after(select)
        //         .run_if(in_state(GameState::Game)),
        // );
        // app.add_systems(
        //     Update,
        //     evade_ball.after(select).run_if(in_state(GameState::Game)),
        // );
        // app.add_systems(
        //     Update,
        //     bat_swing.after(select).run_if(in_state(GameState::Game)),
        // );
    }
}

pub fn load_npc(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut _texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Spawn npc Sprite for testing
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Player4.png"),
            transform: Transform::with_scale(
                Transform::from_xyz(-600., 300., 3.),
                Vec3::splat(0.13),
            ),
            ..default()
        })
        .insert(NPCTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .insert(NPC)
        .insert(NPCVelocity::new())
        .insert(States::Idle)
        .insert(Difficulty { difficulty: 75 })
        .insert(Maps {
            path_map: load_map_path(),
            walkable: load_walkable(),
        })
        .insert(Path {
            path: Vec::with_capacity(1),
            goal: Vec2::splat(-1.),
        })
        .insert(AnimationTimer(Timer::from_seconds(
            ANIM_TIME,
            TimerMode::Repeating,
        )));
    // .insert();
    // .insert(SyncCell {
    //     inner: Tree {
    //         root_node: Node::Null,
    //     },
    // });
    //spawn bat sprite
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Bat.png"),
            transform: Transform::with_scale(Transform::from_xyz(-5., 0., 3.), Vec3::splat(0.13)),
            ..default()
        })
        .insert(NPCBat)
        .insert(NPCTimer(Timer::from_seconds(0.3, TimerMode::Repeating)))
        .insert(AnimationTimer(Timer::from_seconds(
            ANIM_TIME,
            TimerMode::Repeating,
        )));
}

// Select next move
// pub fn select(
//     mut npcs: Query<
//         (&mut Transform, &mut States, &Difficulty, &mut NPCTimer),
//         (With<NPC>, Without<Player>, Without<Ball>),
//     >,
//     player: Query<&Transform, (With<Player>, Without<NPC>, Without<Ball>)>,
//     ball: Query<&Transform, (With<Ball>, Without<Player>, Without<NPC>)>,
//     time: Res<Time>,
// ) {
//     // NPC, Ball, Player Position
//     for (npc_transform, mut state, difficulty, mut timer) in npcs.iter_mut() {
//         for player_transform in player.iter() {
//             for ball_transform in ball.iter() {
//                 let npc_player_distance =
//                     Vec3::distance(npc_transform.translation, player_transform.translation);
//                 let npc_ball_distance =
//                     Vec3::distance(npc_transform.translation, ball_transform.translation);

//                 let mut rand = thread_rng();

//                 // If timer is up, roll next state
//                 timer.tick(time.delta());
//                 if tier.just_finished() {m
//                     // This will be the chance to go to the aggressive state selections
//                     state.to_idle();
//                     let state_flag: i32;

//                     // Calculate proportion of probability equal to aggresion
//                     let agg_factor = difficulty.difficulty as f32 / 100.0;

//                     // Normalize the probabilities (aggression type)/(difficulty amount)
//                     let agg_prob = npc_player_distance + npc_ball_distance;
//                     let agg_prob_player = 1.0 - (npc_player_distance / agg_prob);
//                     let agg_prob_ball = 1.0 - (npc_ball_distance / agg_prob);

//                     // Scale aggression probabilities based on difficulty
//                     let agg_prob_player = agg_prob_player * agg_factor;
//                     let agg_prob_ball = agg_prob_ball * agg_factor;

//                     // % probability of aggression
//                     let agg_prob = agg_prob_ball + agg_prob_player;

//                     let selection = rand.gen_range(0.0..1.0);

//                     if selection <= agg_prob {
//                         state.to_aggression();
//                         state_flag = 0;
//                     } else {
//                         state.to_evade();
//                         state_flag = 1;
//                     }
//                     // Select go to ball or player
//                     if state_flag == 0 {
//                         if agg_prob_ball > agg_prob_player {
//                             state.to_aggression_ball();
//                         } else {
//                             state.to_aggression_player();
//                         }
//                     }
//                     if npc_ball_distance < 100. {
//                         state.to_idle();
//                     }
//                     timer.reset();
//                 }
//             }
//         }
//     }
// }
