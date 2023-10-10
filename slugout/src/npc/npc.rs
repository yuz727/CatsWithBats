use crate::components::*;
use crate::npc::npc_events::*;
use bevy::prelude::*;
use rand::prelude::*;
// Timer for movement
#[derive(Component, Deref, DerefMut)]
pub struct NPCTimer(Timer);

#[derive(Component)]
pub struct NPCVelocity {
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct NPC;

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
    fn to_Idle(&mut self) {
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
        }
    }
}

pub struct NPCPlugin;

impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_npc);
        app.add_systems(Update, select);
        app.add_systems(Update, approach_player.after(select));
        app.add_systems(Update, approach_ball.after(select));
        app.add_systems(Update, evade_ball);
    }
}

pub fn load_npc(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = thread_rng();
    // Spawn npc Sprite for testing
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("crystal_small.png"),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        })
        .insert(NPCTimer(Timer::from_seconds(
            rng.gen_range(0.0..1.0),
            TimerMode::Repeating,
        )))
        .insert(NPC)
        .insert(NPCVelocity::new())
        .insert(States::Idle)
        .insert(Difficulty { difficulty: 50 });
}

// Select next move
pub fn select(
    mut npcs: Query<
        (&mut Transform, &mut States, &Difficulty, &mut NPCTimer),
        (With<NPC>, Without<Player>, Without<Ball>),
    >,
    mut player: Query<&mut Transform, (With<Player>, Without<NPC>, Without<Ball>)>,
    mut ball: Query<&mut Transform, (With<Ball>, Without<Player>, Without<NPC>)>,
    time: Res<Time>,
) {
    // NPC, Ball, Player Position
    let (npc_transform, mut state, difficulty, mut timer) = npcs.single_mut();
    let player_transform = player.single_mut();
    let ball_transform = ball.single_mut();
    let npc_player_distance =
        Vec3::distance(npc_transform.translation, player_transform.translation);
    let npc_ball_distance = Vec3::distance(npc_transform.translation, ball_transform.translation);
    let mut rand = thread_rng();

    // If timer is up, roll next state
    timer.tick(time.delta());
    if timer.just_finished() {
        // This will be the chance to go to the aggressive state selections
        // TODO: Have some kind of formula for calculating the chance.
        state.to_Idle();
        let mut state_flag = -1;
        let aggression_threshold = 7.5;
        let selection = rand.gen_range(0.0..10.0);
        if (0.5 <= selection) && !(selection > aggression_threshold) {
            state.to_aggression();
            state_flag = 0;
        } else if aggression_threshold <= selection {
            state.to_evade();
            state_flag = 1;
        }
        // Select go to ball or player
        if state_flag == 0 {
            let aggression_selection = rand.gen_range(0..10);
            if aggression_selection < 5 {
                state.to_aggression_ball();
            } else {
                state.to_aggression_player();
            }
        }
        timer.reset();
    }
}
