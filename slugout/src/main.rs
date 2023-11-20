use bevy::prelude::*;

mod game;
mod menu;
mod setup;
mod multiplayer;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Setup,
    Game,
    Multiplayer,
    JoinGame,
    HostGame,
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("background1_small.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        })
        .insert(game::components::Background);
    }

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}