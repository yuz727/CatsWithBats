use crate::components::*;
use bevy::{prelude::*, window::PresentMode};

mod game
mod lifecycle

const WIN_W: f32 = 1280.0;
const WIN_H: f32 = 720.0;
const TITLE: &str = "Slugout";

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
        .add_plugins(lifecycle::LifeCyclePlugin)
        .run();
}
