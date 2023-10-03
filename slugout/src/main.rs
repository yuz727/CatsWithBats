use bevy::{prelude::*, window::PresentMode};

mod credits;
mod npc;

const WIN_W: f32 = 1280.0;
const WIN_H: f32 = 720.0;
const TITLE: &str = "Slugout";

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Face;

#[derive(Component)]
pub struct Bat;
#[derive(Component)]
pub struct Object;



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
        
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle{
        texture: asset_server.load("background1_small.png"),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    }).insert(Background);

    // Load Player
    commands.spawn(SpriteBundle{
        texture: asset_server.load("Player.png"),
        transform: Transform::with_scale(Transform::from_xyz(0., 0., 1.), Vec3::splat(0.13)),
        ..default()
    }).insert(Player);

    commands.spawn(SpriteBundle{
        texture: asset_server.load("Face.png"),
        transform: Transform::with_scale(Transform::from_xyz(0., 0., 2.), Vec3::splat(0.13)),
        ..default()
    }).insert(Face);

    commands.spawn(SpriteBundle{
        texture: asset_server.load("Bat.png"),
        transform: Transform::with_scale(Transform::from_xyz(-5., 0., 2.), Vec3::splat(0.13)),
        ..default()
    }).insert(Bat);

    
    // Load Objects
    commands.spawn(SpriteBundle{
        texture: asset_server.load("SideTable.png"),
        transform: Transform::with_rotation(Transform::with_scale(Transform::from_xyz(280.,20., 1.), Vec3::splat(0.18)), Quat::from_rotation_z(5.5)),
        ..default()
    }).insert(Object);

    commands.spawn(SpriteBundle{
        texture: asset_server.load("TVStand.png"),
        transform: Transform::with_rotation(Transform::with_scale(Transform::from_xyz(-300., -150., 1.), Vec3::splat(0.18)), Quat::from_rotation_z(4.)),
        ..default()
    }).insert(Object);

    commands.spawn(SpriteBundle{
        texture: asset_server.load("Recliner.png"),
        transform: Transform::with_rotation(Transform::with_scale(Transform::from_xyz(120., 160., 1.), Vec3::splat(0.18)), Quat::from_rotation_z(0.7)),
        ..default()
    }).insert(Object);
}

