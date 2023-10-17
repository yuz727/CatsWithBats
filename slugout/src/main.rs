use crate::components::*;
use bevy::{prelude::*, window::PresentMode};

mod ball;
mod collisions;
mod components;
mod credits;
mod npc;
mod player;

const WIN_W: f32 = 1280.0;
const WIN_H: f32 = 720.0;
const TITLE: &str = "Slugout";

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        //.add_plugins(credits::credits::CreditsPlugin)
        .add_systems(Startup, setup)
        .add_plugins(npc::npc::NPCPlugin)
        .add_plugins(ball::ball::BallPlugin)
        //.add_plugins(collisions::collisions::CollisionsPlugin)
        .add_systems(Update, player::player_movement::move_player)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("background1_small.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        })
        .insert(Background);

    // Load Player
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Player.png"),
            transform: Transform::with_scale(Transform::from_xyz(0., 0., 2.), Vec3::splat(0.13)),
            ..default()
        })
        .insert(Player)
        .insert(player::player_movement::PlayerVelocity::new())
        .insert(Colliding::new());

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Face.png"),
            transform: Transform::with_scale(Transform::from_xyz(0., 0., 3.), Vec3::splat(0.13)),
            ..default()
        })
        .insert(Face);

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Bat.png"),
            transform: Transform::with_scale(Transform::from_xyz(-5., 0., 3.), Vec3::splat(0.13)),
            ..default()
        })
        .insert(Bat);

    // Load Objects
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("SideTable.png"),
            transform: //Transform::with_rotation(
                Transform::with_scale(Transform::from_xyz(120., 170., 1.), Vec3::splat(0.18)),
                //Quat::from_rotation_z(5.5),
            //),
            ..default()
        })
        .insert(Object);

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("TVStand.png"),
            transform: Transform::with_rotation(
                Transform::with_scale(Transform::from_xyz(0., -250., 1.), Vec3::splat(0.18)),
                Quat::from_rotation_z(4.72),
            ),
            ..default()
        })
        .insert(Object);

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Recliner.png"),
            transform: //Transform::with_rotation(
                Transform::with_scale(Transform::from_xyz(-60., 210., 1.), Vec3::splat(0.18)),
                //Quat::from_rotation_z(0.7),
            //),
            ..default()
        })
        .insert(Object);
}
