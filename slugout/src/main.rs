use bevy::{prelude::*, window::PresentMode};

mod credits;
mod npc;

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
        
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle { 
        texture: asset_server.load("background1_small.png"),
        ..default()
    });
}

