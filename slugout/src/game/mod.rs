use bevy::{prelude::*, window::PresentMode};

use crate::GameState;
use crate::game::components::Aim;

use self::components::{Background, Player, Colliding, Face, Bat, Object, Rug};

mod ball;
pub mod components;
mod npc;
mod npc_events;
mod player_movement;

const WIN_W: f32 = 1280.0;
const WIN_H: f32 = 720.0;
const TITLE: &str = "Slugout";

pub struct GamePlugin; 

impl Plugin for GamePlugin { 
    fn build(&self, app: &mut App) {
        // .insert_resource(ClearColor(Color::rgb(0., 0., 0.)));
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: TITLE.into(),
                    resolution: (WIN_W, WIN_H).into(),
                    present_mode: PresentMode::Fifo,
                    ..default()
                }),
                ..default()
            })
        )
        .add_systems(Startup, setup)
        //.add_plugins(npc::NPCPlugin)
        .add_plugins(ball::BallPlugin)
        .add_systems(Update, player_movement::move_player.run_if(in_state(GameState::Game)));
    }
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    // Load Player
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Player.png"),
            transform: Transform::with_scale(Transform::from_xyz(0., 0., 10.), Vec3::splat(0.13)),
            ..default()
        })
        .insert(Player)
        .insert(player_movement::PlayerVelocity::new())
        .insert(Colliding::new());

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Face.png"),
            transform: Transform::with_scale(Transform::from_xyz(0., 0., 20.), Vec3::splat(0.13)),
            ..default()
        })
        .insert(Face);

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Bat.png"),
            transform: Transform::with_scale(Transform::from_xyz(-5., 0., 20.), Vec3::splat(0.13)),
            ..default()
        })
        .insert(Bat);

    // Load Objects
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("SideTable.png"),
            transform:
                Transform::with_scale(Transform::from_xyz(120., 170., 2.), Vec3::splat(0.18)),
            ..default()
        })
        .insert(Object);

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("TVStand.png"),
            transform: Transform::with_rotation(
                Transform::with_scale(Transform::from_xyz(0., -250., 2.), Vec3::splat(0.18)),
                Quat::from_rotation_z(4.72),
            ),
            ..default()
        })
        .insert(Object);

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Recliner.png"),
            transform:
                Transform::with_scale(Transform::from_xyz(-60., 210., 2.), Vec3::splat(0.18)),
            ..default()
        })
        .insert(Object); 
    commands 
        .spawn(SpriteBundle {
            texture: asset_server.load("Rug.png"),
            transform: Transform::with_rotation(
                Transform::with_scale(Transform::from_xyz(0., 0., 1.), Vec3::splat(0.6)),
                Quat::from_rotation_z(1.56),
            ),
            ..default()
        })
        .insert(Rug{friction: 1.4,}); 
    commands
    .spawn(SpriteBundle {
        texture: asset_server.load("newAim.png"),
        transform: Transform::with_scale(Transform::from_xyz(-2., 0., 4.), Vec3::splat(0.13)),
        ..default()
    })
    .insert(Aim);
    commands.insert_resource(Events::<CursorMoved>::default());
}
