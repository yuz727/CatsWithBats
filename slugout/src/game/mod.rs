use crate::game::components::{Aim, PlayerNumber};
use crate::multiplayer::ClientListVector;
use crate::multiplayer::server::OtherPlayer;
use bevy::{prelude::*, text::FontAtlasSet};
use crate::game::npc::*;

use crate::{despawn_screen, GameState, MultiplayerState, MAP, TEXT_COLOR};

use bevy::input;
use bevy::{prelude::*, window::PresentMode};
use std::io::{stdin, stdout, Write};
use std::string;

use self::components::{Bat, Colliding, Health, Object, Player, Rug, Background};

pub mod ball;
pub mod components;
mod npc;
mod npc_bully;
mod npc_events;
// mod npc_tree;
mod pathfinding;
pub mod player_movement;
mod powerups;


// mod tree;
const WIN_W: f32 = 1280.0;
const WIN_H: f32 = 720.0;
const TITLE: &str = "Slugout";
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub static mut DIFFICULTY: i32 = -1;
pub static mut BULLY_MODE: bool = false;

#[derive(Component)]
struct SelectedOption;

#[derive(Component)]
struct OnDifficultySelectScreen;

#[derive(Component)]
enum SingleplayerButtonAction {
    Game,
    InputProcess,
    Back,
}

#[derive(Component)]
struct InputText(pub String);
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
                Update,
                difficulty_select
                    .after(difficulty_select_setup)
                    .run_if(in_state(GameState::DifficultySelect)),
            )
            .add_systems(
                OnExit(GameState::DifficultySelect),
                despawn_screen::<OnDifficultySelectScreen>,
            )
            .add_plugins(powerups::PowerUpPlugin);
        if unsafe { MAP == 1 } {
            app.add_systems(OnEnter(MultiplayerState::Game), setup_mult)
                .add_systems(OnEnter(GameState::Game), setup)
                .add_systems(OnEnter(GameState::Game), setup_map1)
                .add_systems(OnEnter(MultiplayerState::Game), setup_map1)
                .add_plugins(ball::BallPlugin)
                .add_plugins(npc::NPCPlugin {
                    bully_mode: unsafe { BULLY_MODE },
                })
                .add_systems(
                    Update,
                    player_movement::player_npc_collisions.run_if(in_state(GameState::Game)),
                )
                .add_systems(
                    Update,
                    player_movement::move_player.run_if(in_state(GameState::Game)),
                )
                .add_systems(
                    Update,
                    player_movement::move_player_mult.run_if(in_state(MultiplayerState::Game)),
                )
                .add_systems(Update, end_game.run_if(in_state(GameState::Game)));
        } else if unsafe { MAP == 2 || MAP == 3 } {
            app.add_systems(OnEnter(MultiplayerState::Game), setup)
                .add_systems(OnEnter(GameState::Game), setup)
                .add_plugins(ball::BallPlugin)
                .add_plugins(npc::NPCPlugin {
                    bully_mode: unsafe { BULLY_MODE },
                })
                .add_systems(
                    Update,
                    player_movement::move_player.run_if(in_state(GameState::Game)),
                )
                .add_systems(
                    Update,
                    player_movement::player_npc_collisions.run_if(in_state(GameState::Game)),
                )
                .add_systems(
                    Update,
                    player_movement::move_player_mult.run_if(in_state(MultiplayerState::Game)),
                )
                .add_systems(Update, end_game.run_if(in_state(GameState::Game)));
        } else if unsafe { MAP == 4 } {
            app.add_systems(OnEnter(GameState::Game), setup)
                .add_systems(OnEnter(MultiplayerState::Game), setup)
                .add_plugins(ball::BallPlugin)
                .add_plugins(npc::NPCPlugin {
                    bully_mode: unsafe { BULLY_MODE },
                })
                .add_systems(OnEnter(GameState::Game), setup_map4)
                .add_systems(OnEnter(MultiplayerState::Game), setup_map4)
                .add_systems(
                    Update,
                    player_movement::move_player.run_if(in_state(GameState::Game)),
                )
                .add_systems(
                    Update,
                    player_movement::player_npc_collisions.run_if(in_state(GameState::Game)),
                )
                .add_systems(
                    Update,
                    player_movement::move_player_mult.run_if(in_state(MultiplayerState::Game)),
                )
                .add_systems(Update, end_game.run_if(in_state(GameState::Game)));
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
        .insert(Player {
            powerup: "none".to_string(),
            powerup_timer: 0.,
            health: 3,
            health_timer: 0.,

        })
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
    /*commands
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
        .insert(Health);*/
}

pub fn setup_mult(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut client_list: ResMut<ClientListVector>,
    player_number: Res<crate::multiplayer::PlayerNumber>,
) {

    println!("is this function running ");
    // Load Player
    for client in client_list.0.iter_mut() {
        let player_x = client.player_info.as_mut().unwrap().position.0;
        let player_y = client.player_info.as_mut().unwrap().position.1;
        if player_number.0
            == client.username[4..client.username.len()]
                .parse::<u32>()
                .unwrap()
        {
            println!("spawn 1");
            commands
                .spawn(SpriteBundle {
                    texture: asset_server.load(if client.username.contains("1") {
                        "Player.png"
                    } else if client.username.contains("2") {
                        "Player2.png"
                    } else if client.username.contains("3") {
                        "Player3.png"
                    } else {
                        "Player4.png"
                    }),
                    transform: Transform::with_scale(
                        Transform::from_xyz(player_x, player_y, 10.),
                        Vec3::splat(0.13),
                    ),
                    ..default()
                })
                .insert(Player {
                    powerup: "none".to_string(),
                    powerup_timer: 0.,
                    health: 3,
                    health_timer: 0.,
                })
                .insert(player_movement::PlayerVelocity::new())
                .insert(Colliding::new())
                .insert(PlayerNumber {
                    number: client.username[4..client.username.len()]
                        .parse::<usize>()
                        .unwrap(),
                });

            // commands
            //     .spawn(SpriteBundle {
            //         texture: asset_server.load("Face.png"),
            //         transform: Transform::with_scale(Transform::from_xyz(player_x, player_y, 20.), Vec3::splat(0.13)),
            //         ..default()
            //     })
            //     .insert(Face)
            //     .insert(Player{
            //         powerup: "none".to_string(),
            //         powerup_timer: 0.,
            // });

            commands
                .spawn(SpriteBundle {
                    texture: asset_server.load("Bat.png"),
                    transform: Transform::with_scale(
                        Transform::from_xyz(player_x - 5., player_y, 20.),
                        Vec3::new(0.175, 0.175, 0.),
                    ),
                    ..default()
                })
                .insert(Bat)
                .insert(Player {
                    powerup: "none".to_string(),
                    powerup_timer: 0.,
                    health: 3, 
                    health_timer: 0.,
                })
                .insert(PlayerNumber {
                    number: client.username[4..client.username.len()]
                        .parse::<usize>()
                        .unwrap(),
                });
        } else {
            println!("spawn 2");
            commands
                .spawn(SpriteBundle {
                    texture: asset_server.load(if client.username.contains("1") {
                        "Player.png"
                    } else if client.username.contains("2") {
                        "Player2.png"
                    } else if client.username.contains("3") {
                        "Player3.png"
                    } else {
                        "Player4.png"
                    }),
                    transform: Transform::with_scale(
                        Transform::from_xyz(player_x, player_y, 10.),
                        Vec3::splat(0.13),
                    ),
                    ..default()
                })
                .insert(crate::multiplayer::server::OtherPlayer)
                .insert(player_movement::PlayerVelocity::new())
                .insert(Colliding::new())
                .insert(PlayerNumber {
                    number: client.username[4..client.username.len()]
                        .parse::<usize>()
                        .unwrap(),
                });
            // commands
            // .spawn(SpriteBundle {
            //     texture: asset_server.load("Face.png"),
            //     transform: Transform::with_scale(Transform::from_xyz(player_x, player_y, 20.), Vec3::splat(0.13)),
            //     ..default()
            // })
            // .insert(Face)
            // .insert(crate::multiplayer::server::OtherPlayer);

            commands
                .spawn(SpriteBundle {
                    texture: asset_server.load("Bat.png"),
                    transform: Transform::with_scale(
                        Transform::from_xyz(player_x - 5., player_y, 20.),
                        Vec3::new(0.175, 0.175, 0.),
                    ),
                    ..default()
                })
                .insert(Bat)
                
                .insert(crate::multiplayer::server::OtherPlayer)
                .insert(PlayerNumber {
                    number: client.username[4..client.username.len()]
                        .parse::<usize>()
                        .unwrap(),
                });
        }
    }

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
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("newAim.png"),
            transform: Transform::with_scale(Transform::from_xyz(-2., 0., 4.), Vec3::splat(0.13)),
            ..default()
        })
        .insert(Aim);
    commands.insert_resource(Events::<CursorMoved>::default());
    // commands
    //     .spawn(SpriteBundle {
    //         texture: asset_server.load("FullHealth.png"),
    //         transform: Transform::with_scale(
    //             Transform::from_xyz(525., 280., 2.),
    //             Vec3::splat(0.11),
    //         ),
    //         ..default()
    //     })
    //     .insert(Health);
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

fn setup_map4(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Coral.png"),
            transform: Transform::from_xyz(0., 180., 2.),
            ..default()
        })
        .insert(Object);
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Coral.png"),
            transform: Transform::from_xyz(0., -180., 2.),
            ..default()
        })
        .insert(Object);
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Coral.png"),
            transform: Transform::from_xyz(-320., 180., 2.),
            ..default()
        })
        .insert(Object);
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Coral.png"),
            transform: Transform::from_xyz(-320., -180., 2.),
            ..default()
        })
        .insert(Object);
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Coral.png"),
            transform: Transform::from_xyz(320., 180., 2.),
            ..default()
        })
        .insert(Object);
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Coral.png"),
            transform: Transform::from_xyz(320., -180., 2.),
            ..default()
        })
        .insert(Object);
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
                .spawn(
                    TextBundle::from_section(
                        String::new().to_string(),
                        TextStyle {
                            font_size: 30.0,
                            color: TEXT_COLOR,
                            ..default()
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    }),
                )
                .insert(InputText(String::new()));
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    SingleplayerButtonAction::InputProcess,
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

fn difficulty_select(
    mut char_input_events: EventReader<ReceivedCharacter>,
    keyboard: Res<Input<KeyCode>>,
    mut textbox: Query<(&mut Text, &mut InputText), With<InputText>>,
) {
    for (mut single_box, mut user_input) in textbox.iter_mut() {
        for event in char_input_events.iter() {
            if keyboard.pressed(KeyCode::Back) {
                single_box.sections[0].value.pop();
                user_input.0.pop();
            } else {
                single_box.sections[0].value.push(event.char);
                user_input.0.push(event.char);
            }
        }
    }
}

fn single_player_menu_action(
    interaction_query: Query<
        (&Interaction, &SingleplayerButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
    mut textbox: Query<&InputText, With<InputText>>,
) {
    for (interaction, multiplayer_button_action) in &interaction_query {
        for user_input in textbox.iter_mut() {
            if *interaction == Interaction::Pressed {
                match multiplayer_button_action {
                    SingleplayerButtonAction::Game => {
                        game_state.set(GameState::Game);
                    }
                    SingleplayerButtonAction::InputProcess => {
                        let temp: &String = &user_input.0;
                        if process_difficulty_input(temp) {
                            game_state.set(GameState::Game);
                        }
                    }
                    SingleplayerButtonAction::Back => {
                        game_state.set(GameState::Setup);
                    }
                }
            }
        }
    }
}

fn process_difficulty_input(input_text: &String) -> bool {
    match input_text.parse::<i32>() {
        Ok(i) => {
            if i > 0 && i <= 5 {
                unsafe { DIFFICULTY = i * 20 };
                return true;
            } else {
                println!("BULLY MODE ACTIVATED!");
                unsafe { DIFFICULTY = 100 };
                unsafe { BULLY_MODE = true };
                return true;
            }
        }
        Err(..) => {
            println!("Invalid Difficulty, Try Again");
            return false;
        }
    };
    // return false;
}

fn end_game (
    mut commands: Commands,
    npc_query: Query<(&NPC, Entity), (With<NPC>, Without<Player>)>,
    player_query: Query<(&Player, Entity), (With<Player>, Without<NPC>)>,
    mut game_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
){
    let font_handle = asset_server.load("FiraSans-Bold.ttf");
    let (npc, npc_entity) = npc_query.single();
    let(player, player_entity) = player_query.single();

    if npc.health == 0 || player.health == 0 {
        commands.entity(npc_entity).despawn();
        commands.entity(player_entity).despawn();

        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("background1_small.png"),
                transform: Transform::from_xyz(0., 0., 300.),
                ..default()
            })
            .insert(Background);
        
        game_state.set(GameState::GameOver);
        
        if npc.health == 0{
            commands
            .spawn(TextBundle {
                text: Text::from_section(
                    "Game Over. You Win.",
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: 100.0,
                        color: Color::WHITE,
                    },
                ),
                transform: Transform::from_xyz(600., 350., 400.),
                ..default()
            });
        }else{
            commands
            .spawn(TextBundle {
                text: Text::from_section(
                    "Game Over. You Lose.",
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: 100.0,
                        color: Color::WHITE,
                    },
                ),
                transform: Transform::from_xyz(600., 350., 400.),
                ..default()
            });
        }

    }

}

pub fn end_game_mult (
    mut commands: Commands,
    player_query: Query<(&Player, Entity), (With<Player>)>,
    other_players: Query<(&PlayerNumber, Entity), (With<OtherPlayer>, Without<Bat>)>,
    mut game_state: ResMut<NextState<GameState>>,
    mut multiplayer_state: ResMut<NextState<MultiplayerState>>,
    asset_server: Res<AssetServer>,
    mut client_list_vec: ResMut<ClientListVector>,
    player_number: Res<crate::multiplayer::PlayerNumber>,
){
    
    for player in client_list_vec.0.iter() { 

        
        if player.username.contains(player_number.0.to_string().as_str()) && player.player_info.as_ref().unwrap().health == 0 
        {
            //is me, despawn game over screen
            for (_player, player_entity) in player_query.iter()
            {
                commands.entity(player_entity).despawn();
            }
            
            game_state.set(GameState::GameOver);
            multiplayer_state.set(MultiplayerState::Disabled);
            commands
            .spawn(SpriteBundle {
                texture: asset_server.load("background1_small.png"),
                transform: Transform::from_xyz(0., 0., 300.),
                ..default()
            })
            .insert(Background);


        }
        else if player.player_info.as_ref().unwrap().health == 0 {
            //is not me, despawn player
            for (player_number, other_player_entity) in other_players.iter() {
                if  player.username.contains(player_number.number.to_string().as_str()) {
                    commands.entity(other_player_entity).despawn();
                }
            }
        }
        
    }

   

}

