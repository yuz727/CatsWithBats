use bevy::{prelude::*, window::PresentMode};

mod game;
mod menu;
mod setup;

const WIN_W: f32 = 1280.0;
const WIN_H: f32 = 720.0;
const TITLE: &str = "Slugout";
const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Setup,
    Game
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins(game::GamePlugin)
        .add_plugins(menu::MenuPlugin)
        .add_plugins(setup::SetupPlugin)
        .run();
}
// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}