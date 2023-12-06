use bevy::prelude::*;
use std::io::{stdin, stdout, Write};

mod game;
mod menu;
mod multiplayer;
mod setup;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub static mut MAP: i32 = -1;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Setup,
    Game,
    Multiplayer,
    JoinGame,
    HostGame,
    DifficultySelect,
    GameOver,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MultiplayerState {
    Main,
    Lobby,
    Game,
    #[default]
    Disabled,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum NetworkingState {
    Host,
    Join,
    #[default]
    Disabled,
}

fn main() {
    map_picking();
    App::new()
        .add_state::<GameState>()
        .add_state::<MultiplayerState>()
        .add_systems(Startup, setup)
        .add_plugins(game::GamePlugin)
        .add_plugins(menu::MenuPlugin)
        .add_plugins(setup::SetupPlugin)
        .add_plugins(multiplayer::MultiplayerPlugin)
        .run();
}

/// Spawning In Background, Map image is decided depending on the map selected.
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    if unsafe { MAP } == 1 {
        commands.spawn(Camera2dBundle::default());
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("background1_small.png"),
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            })
            .insert(game::components::Background);
    } else if unsafe { MAP } == 2 {
        commands.spawn(Camera2dBundle::default());
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("background2_small.png"),
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            })
            .insert(game::components::Background);
    } else if unsafe { MAP } == 3 {
        commands.spawn(Camera2dBundle::default());
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("background3_small.png"),
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            })
            .insert(game::components::Background);
    } else if unsafe { MAP } == 4 {
        commands.spawn(Camera2dBundle::default());
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("background4_small.png"),
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            })
            .insert(game::components::Background);
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

// Prompt User Input in Console to Pick a Map
fn map_picking() {
    loop {
        let mut input = String::new();
        print!("Pick a Map number from 1 - 4: ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");
        let trimmed = input.trim();
        match trimmed.parse::<i32>() {
            Ok(i) => {
                if i > 0 && i <= 4 {
                    unsafe { MAP = i };
                    break;
                } else {
                    println!("Invalid Map Number, Try Again");
                }
            }
            Err(..) => println!("Invalid Map Number, Try Again"),
        };
    }
}
