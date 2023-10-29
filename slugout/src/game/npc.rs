// use crate::components::*;

use crate::game::npc_events::*;
use crate::GameState;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use rand::prelude::*;

use super::components::Ball;
use super::components::Player;

const PLAYER_SIZE: f32 = 30.;
// Timer for movement
#[derive(Component, Deref, DerefMut)]
pub struct NPCTimer(Timer);

#[derive(Component)]
struct SwingAnimation {
    time: Stopwatch,
}

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
    AggressionBall,
    AggressionPlayer,
}

#[derive(Component)]
pub struct Difficulty {
    difficulty: i32,
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
    fn to_aggression_ball(&mut self) {
        *self = match std::mem::replace(self, States::AggressionBall) {
            States::Aggression => States::AggressionBall,
            v => v,
        }
    }
    fn to_aggression_player(&mut self) {
        *self = match std::mem::replace(self, States::AggressionPlayer) {
            States::Aggression => States::AggressionPlayer,
            v => v,
        }
    }
    fn to_idle(&mut self) {
        *self = match std::mem::replace(self, States::Idle) {
            States::Aggression => States::Idle,
            States::AggressionBall => States::Idle,
            States::AggressionPlayer => States::Idle,
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

// impl From<Vec3> for DetectionRadius {
//     fn from(pos: Vec3) -> Self {
//         Self {
//             top: pos.y + PLAYER_SIZE / 2.,
//             bottom: pos.y - PLAYER_SIZE / 2.,
//             left: pos.x - PLAYER_SIZE / 2.,
//             right: pos.x + PLAYER_SIZE / 2.,
//         }
//     }
// }
pub struct NPCPlugin;

impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), load_npc);
        app.add_systems(Update, select.run_if(in_state(GameState::Game)));
        app.add_systems(Update, avoid_collision.run_if(in_state(GameState::Game)));
<<<<<<< HEAD
        app.add_systems(Update, approach_player.after(select).run_if(in_state(GameState::Game)));
        app.add_systems(Update, approach_ball.after(select).run_if(in_state(GameState::Game)));
        app.add_systems(Update, evade_ball.after(select).run_if(in_state(GameState::Game)));
        //app.add_systems(Update, bat_swing.after(select).run_if(in_state(GameState::Game)));
=======
        app.add_systems(
            Update,
            approach_player
                .after(select)
                .run_if(in_state(GameState::Game)),
        );
        app.add_systems(
            Update,
            approach_ball
                .after(select)
                .run_if(in_state(GameState::Game)),
        );
        app.add_systems(
            Update,
            evade_ball.after(select).run_if(in_state(GameState::Game)),
        );
        app.add_systems(
            Update,
            bat_swing.after(select).run_if(in_state(GameState::Game)),
        );
>>>>>>> d8078cac510e89f8fc01c40371a4c48ffdc1f7e7
    }
}

pub fn load_npc(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = thread_rng();
    // Spawn npc Sprite for testing
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Player4.png"),
            transform: Transform::with_scale(Transform::from_xyz(0., 0., 3.), Vec3::splat(0.13)),
            ..default()
        })
        .insert(NPCTimer(Timer::from_seconds(
            rng.gen_range(0.0..1.0),
            TimerMode::Repeating,
        )))
        .insert(NPC)
        .insert(NPCVelocity::new())
        .insert(States::Idle)
        .insert(Difficulty { difficulty: 75 });

    //spawn bat sprite
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Bat.png"),
            transform: Transform::with_scale(Transform::from_xyz(-5., 0., 3.), Vec3::splat(0.13)),
            ..default()
        })
        .insert(NPCBat)
        .insert(NPCTimer(Timer::from_seconds(0.3, TimerMode::Repeating)));

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Face.png"),
            transform: Transform::with_scale(Transform::from_xyz(0., 0., 5.), Vec3::splat(0.13)),
            ..default()
        })
        .insert(NPCFace);
}

// Select next move
pub fn select(
    mut npcs: Query<
        (&mut Transform, &mut States, &Difficulty, &mut NPCTimer),
        (With<NPC>, Without<Player>, Without<Ball>),
    >,
    player: Query<&Transform, (With<Player>, Without<NPC>, Without<Ball>)>,
    ball: Query<&Transform, (With<Ball>, Without<Player>, Without<NPC>)>,
    time: Res<Time>,
) {
    // NPC, Ball, Player Position
    for (npc_transform, mut state, difficulty, mut timer) in npcs.iter_mut() {
        for player_transform in player.iter() {
            for ball_transform in ball.iter() {
                let npc_player_distance =
                    Vec3::distance(npc_transform.translation, player_transform.translation);
                let npc_ball_distance =
                    Vec3::distance(npc_transform.translation, ball_transform.translation);

                let mut rand = thread_rng();

                // If timer is up, roll next state
                timer.tick(time.delta());
                if timer.just_finished() {
                    // This will be the chance to go to the aggressive state selections
                    state.to_idle();
                    let mut state_flag = -1;

                    // Calculate proportion of probability equal to aggresion
                    let agg_factor = difficulty.difficulty as f32 / 100.0;

                    // Normalize the probabilities (aggression type)/(difficulty amount)
                    let agg_prob = npc_player_distance + npc_ball_distance;
                    let agg_prob_player = 1.0 - (npc_player_distance / agg_prob);
                    let agg_prob_ball = 1.0 - (npc_ball_distance / agg_prob);

                    // Scale aggression probabilities based on difficulty
                    let agg_prob_player = agg_prob_player * agg_factor;
                    let agg_prob_ball = agg_prob_ball * agg_factor;

                    // % probability of aggression
                    let agg_prob = agg_prob_ball + agg_prob_player;

                    let selection = rand.gen_range(0.0..1.0);

                    if selection <= agg_prob {
                        state.to_aggression();
                        state_flag = 0;
                    } else {
                        state.to_evade();
                        state_flag = 1;
                    }
                    // Select go to ball or player
                    if state_flag == 0 {
                        if agg_prob_ball > agg_prob_player {
                            state.to_aggression_ball();
                        } else {
                            state.to_aggression_player();
                        }
                    }
                    if npc_ball_distance < 100. {
                        state.to_idle();
                    }
                    timer.reset();
                }
            }
        }
    }
}
