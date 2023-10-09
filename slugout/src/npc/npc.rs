use crate::components::*;
use crate::npc::npc_events::*;
use bevy::prelude::*;
use rand::prelude::*;
// Timer for movement
#[derive(Component, Deref, DerefMut)]
pub struct MovementTimer(Timer);

#[derive(Component)]
pub struct NPCVelocity {
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct NPC;

impl NPCVelocity {
    fn new() -> Self {
        Self {
            velocity: Vec2::splat(0.),
        }
    }
}

enum States {
    Aggression,
    Evade,
    Idle,
}
pub struct NPCPlugin;

impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_npc);
        //app.add_systems(Update, approach_player);
        app.add_systems(Update, approach_ball);
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
        .insert(MovementTimer(Timer::from_seconds(
            rng.gen_range(0.0..5.0),
            TimerMode::Repeating,
        )))
        .insert(NPC)
        .insert(NPCVelocity::new());
}

pub fn select(
    mut commands: Commands,
    mut npcs: Query<&mut Transform, (With<NPC>, Without<Player>, Without<Ball>)>,
    mut player: Query<&mut Transform, (With<Player>, Without<NPC>, Without<Ball>)>,
    mut ball: Query<&mut Transform, (With<Ball>, Without<Player>, Without<NPC>)>,
    time: Res<Time>,
) {
    let npc_transform = npcs.single_mut();
    let player_transform = player.single_mut();
    let ball_transform = ball.single_mut();
}
