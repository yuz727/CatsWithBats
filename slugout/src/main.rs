use bevy::{prelude::*, window::PresentMode};

mod credits;

const WIN_W: f32 = 1280.0;
const WIN_H: f32 = 720.0;
const TITLE: &str = "Slugout";

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Background;

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
        .add_systems(Startup, setup)
        .add_systems(Startup, credits::credits::load_credit_texture)
        .add_systems(Update, credits::credits::cycle_credits)
        .run();
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Load Background
    commands.spawn(SpriteBundle{
        texture: _asset_server.load("background1_small.png"),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    }).insert(Background);

    // Load Player
    commands.spawn(SpriteBundle{
        texture: _asset_server.load("Player.png"),
        transform: Transform::with_scale(Transform::from_xyz(0., 0., 0.), Vec3::splat(0.2)),
        ..default()
    }).insert(Player);
}

