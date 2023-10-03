use bevy::prelude::*;
use rand::prelude::*;
use crate::npc::npc_events::rand_movement;

// Timer for movement
#[derive(Component, Deref, DerefMut)]
pub struct MovementTimer(Timer);

#[derive(Component)]
pub struct Velocity {
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct NPC;

impl Velocity {
    fn new() -> Self {
        Self {
            velocity: Vec2::splat(0.),
        }
    }
}
pub struct NPCPlugin;

impl Plugin for NPCPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, load_npc);
        app.add_systems(Update, rand_movement);
    }
}

pub fn load_npc(mut commands: Commands, asset_server: Res<AssetServer>){
    let mut rng = thread_rng();
    // Spawn npc Sprite for testing
    commands.spawn(SpriteBundle{
        texture: asset_server.load("crystal_small.png"),
        transform: Transform::from_xyz(0.,0., 1.),
        ..default()
    })  .insert(MovementTimer(Timer::from_seconds(rng.gen_range(0.0..5.0), TimerMode::Repeating)))
        .insert(NPC)
        .insert(Velocity::new());
}