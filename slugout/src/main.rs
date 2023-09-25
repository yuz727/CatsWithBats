use std::process::id;

use bevy::{prelude::*, window::PresentMode, utils::petgraph::visit::EdgeRef};

mod credits;
#[derive(Component, Deref, DerefMut)]
struct PopupTimer(Timer);

const WIN_W: f32 = 1280.0;
const WIN_H: f32 = 720.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 1., 1.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Test".into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
    //    .add_systems(Update, credits::credits::load_credits)
        .add_systems(Update, load_credits)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("random.png"),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("Crystal.png"),
        transform: Transform::from_xyz(0., 0., -1.),
        ..default()
    }).insert(PopupTimer(Timer::from_seconds(0., TimerMode::Once)));
}

fn load_credits(mut update_screen: Query<(&mut Transform, &mut PopupTimer)>, keyboard: Res<Input<KeyCode>>){
    if keyboard.just_pressed(KeyCode::Space) {
        for (mut transform, _timer) in update_screen.iter_mut(){
            transform.translation.z = 1.0;
            info!("Test");
        }
    }
}