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
        .add_systems(Update, move_player)
        .add_systems(Update, move_face)
        .add_systems(Update, move_bat)
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

fn move_player(input: Res<Input<KeyCode>>, mut player: Query<&mut Transform, With<Player>>){
    let mut player_transform = player.single_mut();

    let mut x_vel = 0.;
    let mut y_vel = 0.;

    if input.pressed(KeyCode::A){
        x_vel -= 1.;
    }
    if input.pressed(KeyCode::D){
        x_vel += 1.;
    }
    if input.pressed(KeyCode::W){
        y_vel += 1.;
    }
    if input.pressed(KeyCode::S){
        y_vel -= 1.;
    }

    player_transform.translation.x += x_vel;
    player_transform.translation.y += y_vel;

}

fn move_face(input: Res<Input<KeyCode>>, mut face: Query<&mut Transform, With<Face>>){
    let mut face_transform = face.single_mut();

    let mut x_vel = 0.;
    let mut y_vel = 0.;

    if input.pressed(KeyCode::A){
        x_vel -= 1.;
    }
    if input.pressed(KeyCode::D){
        x_vel += 1.;
    }
    if input.pressed(KeyCode::W){
        y_vel += 1.;
    }
    if input.pressed(KeyCode::S){
        y_vel -= 1.;
    }

    face_transform.translation.x += x_vel;
    face_transform.translation.y += y_vel;

}

fn move_bat(input: Res<Input<KeyCode>>, mut bat: Query<&mut Transform, With<Bat>>){
    let mut bat_transform = bat.single_mut();

    let mut x_vel = 0.;
    let mut y_vel = 0.;

    if input.pressed(KeyCode::A){
        x_vel -= 1.;
    }
    if input.pressed(KeyCode::D){
        x_vel += 1.;
    }
    if input.pressed(KeyCode::W){
        y_vel += 1.;
    }
    if input.pressed(KeyCode::S){
        y_vel -= 1.;
    }

    bat_transform.translation.x += x_vel;
    bat_transform.translation.y += y_vel;

}