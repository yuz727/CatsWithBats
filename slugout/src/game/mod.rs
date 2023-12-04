use crate::game::components::Aim;
use crate::{despawn_screen, GameState, MultiplayerState, MAP, TEXT_COLOR};
use bevy::{app::AppExit, prelude::*, window::PresentMode};
use std::io::{stdin, stdout, Write};

use self::components::{Bat, Colliding, Health, Object, Player, Rug};

mod ball;
pub mod components;
mod npc;
mod npc_bully;
mod npc_events;
// mod npc_tree;
mod pathfinding;
mod player_movement;
// mod tree;
const WIN_W: f32 = 1280.0;
const WIN_H: f32 = 720.0;
const TITLE: &str = "Slugout";
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub static mut DIFFICULTY: i32 = -1;

#[derive(Component)]
struct SelectedOption;

#[derive(Component)]
struct OnDifficultySelectScreen;

#[derive(Component)]
enum SingleplayerButtonAction {
    Game,
    Back,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // .insert_resource(ClearColor(Color::rgb(0., 0., 0.)));
        app.add_state::<GameState>()
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: TITLE.into(),
                    resolution: (WIN_W, WIN_H).into(),
                    present_mode: PresentMode::Fifo,
                    ..default()
                }),
                ..default()
            }))
            .add_systems(
                Update,
                (button_system, single_player_menu_action)
                    .run_if(in_state(GameState::DifficultySelect)),
            )
            .add_systems(
                OnEnter(GameState::DifficultySelect),
                difficulty_select_setup,
            )
            .add_systems(
                OnEnter(GameState::DifficultySelect),
                difficulty_select.after(difficulty_select_setup),
            )
            .add_systems(
                OnExit(GameState::DifficultySelect),
                despawn_screen::<OnDifficultySelectScreen>,
            );
        if unsafe { MAP == 1 } {
            app.add_systems(OnEnter(MultiplayerState::Game), setup)
                .add_systems(OnEnter(GameState::Game), setup)
                .add_systems(OnEnter(GameState::Game), setup_map1)
                .add_systems(OnEnter(MultiplayerState::Game), setup_map1)
                .add_plugins(ball::BallPlugin)
                .add_plugins(npc::NPCPlugin { bully_mode: false })
                .add_systems(
                    Update,
                    player_movement::move_player.run_if(in_state(GameState::Game)),
                )
                .add_systems(
                    Update,
                    player_movement::move_player_mult.run_if(in_state(MultiplayerState::Game)),
                );
        } else if unsafe { MAP == 2 || MAP == 3 } {
            app.add_systems(OnEnter(MultiplayerState::Game), setup)
                .add_systems(OnEnter(GameState::Game), setup)
                .add_plugins(ball::BallPlugin)
                .add_plugins(npc::NPCPlugin { bully_mode: false })
                .add_systems(
                    Update,
                    player_movement::move_player.run_if(in_state(GameState::Game)),
                )
                .add_systems(
                    Update,
                    player_movement::player_NPC_collisions.run_if(in_state(GameState::Game)),
                )
                .add_systems(
                    Update,
                    player_movement::move_player_mult.run_if(in_state(MultiplayerState::Game)),
                );
        } else if unsafe { MAP == 4 } {
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load Player
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Player.png"),
            transform: Transform::with_scale(Transform::from_xyz(0., 0., 10.), Vec3::splat(0.13)),
            ..default()
        })
        .insert(Player)
        .insert(player_movement::PlayerVelocity::new())
        .insert(Colliding::new());

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Bat.png"),
            transform: Transform::with_scale(
                Transform::from_xyz(-5., 0., 20.),
                Vec3::new(0.175, 0.175, 0.),
            ),
            ..default()
        })
        .insert(Bat);

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("newAim.png"),
            transform: Transform::with_scale(Transform::from_xyz(-2., 0., 4.), Vec3::splat(0.13)),
            ..default()
        })
        .insert(Aim);
    commands.insert_resource(Events::<CursorMoved>::default());
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("FullHealth.png"),
            transform: Transform::with_scale(
                Transform::from_xyz(525., 280., 2.),
                Vec3::splat(0.11),
            ),
            ..default()
        })
        .insert(Health);
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("FullHealthNPC.png"),
            transform: Transform::with_scale(
                Transform::from_xyz(-525., 280., 2.),
                Vec3::splat(0.11),
            ),
            ..default()
        })
        .insert(Health);
}

fn setup_map1(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load Objects
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("SideTable.png"),
            transform: Transform::with_scale(
                Transform::from_xyz(120., 170., 2.),
                Vec3::splat(0.18),
            ),
            ..default()
        })
        .insert(Object);

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("TVStand.png"),
            transform: Transform::with_rotation(
                Transform::with_scale(Transform::from_xyz(0., -250., 2.), Vec3::splat(0.18)),
                Quat::from_rotation_z(4.72),
            ),
            ..default()
        })
        .insert(Object);

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Recliner.png"),
            transform: Transform::with_scale(
                Transform::from_xyz(-60., 210., 2.),
                Vec3::splat(0.18),
            ),
            ..default()
        })
        .insert(Object);
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Rug.png"),
            transform: Transform::with_rotation(
                Transform::with_scale(Transform::from_xyz(0., 0., 1.), Vec3::splat(0.6)),
                Quat::from_rotation_z(1.56),
            ),
            ..default()
        })
        .insert(Rug { friction: 1.4 });
}

//This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

fn difficulty_select_setup(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnDifficultySelectScreen,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Select an AI Difficulty from 1 to 5",
                    TextStyle {
                        font_size: 30.0,
                        color: TEXT_COLOR,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            );
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    SingleplayerButtonAction::Game,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Run.", button_text_style.clone()));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    SingleplayerButtonAction::Back,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Back", button_text_style.clone()));
                });
        });
}

fn difficulty_select(_kbd: Res<Input<KeyCode>>) {
    loop {
        let mut input = String::new();
        print!("Pick a Difficulty from 1 - 5: ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");
        let trimmed = input.trim();
        match trimmed.parse::<i32>() {
            Ok(i) => {
                if i > 0 && i <= 5 {
                    unsafe { DIFFICULTY = i * 20 };
                    break;
                } else {
                    println!("Invalid difficulty, try again");
                }
            }
            Err(..) => println!("Invalid difficulty, try again"),
        };
    }
}

fn single_player_menu_action(
    interaction_query: Query<
        (&Interaction, &SingleplayerButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, multiplayer_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match multiplayer_button_action {
                SingleplayerButtonAction::Game => {
                    game_state.set(GameState::Game);
                }
                SingleplayerButtonAction::Back => {
                    game_state.set(GameState::Setup);
                }
            }
        }
    }
}
