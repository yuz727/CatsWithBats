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
}

#[derive(Component)]
pub struct Difficulty {
    difficulty: i32,
}

impl States {
    fn to_aggression(&mut self) {
        *self = match std::mem::replace(self, States::Evade) {
            States::Idle => States::Evade,
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
            rng.gen_range(0.0..0.5),
            TimerMode::Repeating,
        )))
        .insert(NPC)
        .insert(NPCVelocity::new())
        .insert(States::Idle)
        .insert(Difficulty { difficulty: 50 });
}

pub fn select(
    mut npcs: Query<
        (&mut Transform, &mut States, &Difficulty, &mut NPCTimer),
        (With<NPC>, Without<Player>, Without<Ball>),
    >,
    mut player: Query<&mut Transform, (With<Player>, Without<NPC>, Without<Ball>)>,
    mut ball: Query<&mut Transform, (With<Ball>, Without<Player>, Without<NPC>)>,
    time: Res<Time>,
) {
    let (npc_transform, mut state, difficulty, mut timer) = npcs.single_mut();
    let player_transform = player.single_mut();
    let ball_transform = ball.single_mut();
    let npc_player_distance =
        Vec3::distance(npc_transform.translation, player_transform.translation);
    let npc_ball_distance = Vec3::distance(npc_transform.translation, ball_transform.translation);
    let mut rand = thread_rng();
    state.to_aggression();
}
