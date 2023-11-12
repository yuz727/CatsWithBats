use bevy::{app::AppExit, prelude::*, window::ReceivedCharacter};
use std::thread;
use super::{despawn_screen, GameState, TEXT_COLOR};
use std::net::{UdpSocket, SocketAddr};

pub struct MultiplayerPlugin;

mod server;
mod client;

#[derive(Component)]
enum MultiplayerButtonAction {
    HostGame,
    JoinGame,
    Multiplayer,
    Back,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MultiplayerState {
    Main,
    Lobby,
    #[default]
    Disabled,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum NetworkingState{
    Host,
    Join,
    #[default]
    Disabled,
}

#[derive(Component)]
enum LobbyButtonAction {
    Back,
}
#[derive(Component)]
struct SelectedOption;


#[derive(Resource)]
pub struct SocketAddress(pub String);

#[derive(Resource)]
pub struct ClientSocket(pub Option<UdpSocket>);

#[derive(Resource)]
pub struct ServerSocket(pub Option<UdpSocket>);


#[derive(Component)]
struct InputText(pub String);

pub static mut USER_INPUT: Option<String> = None;

impl Plugin for MultiplayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<MultiplayerState>()
        .add_state::<NetworkingState>()
        .add_systems(OnEnter(GameState::Multiplayer), multiplayer_setup)
        .add_systems(OnEnter(MultiplayerState::Main), multiplayer_setup)
        .add_systems(OnExit(MultiplayerState::Disabled), despawn_screen::<OnMultiplayerScreen>)
        .add_systems(OnExit(MultiplayerState::Lobby), despawn_screen::<OnLobbyScreen>)
        .add_systems(OnEnter(MultiplayerState::Lobby), lobby_setup)
        .add_systems(
            Update,
            (multiplayer_menu_action, button_system, update_user_input).run_if(in_state(GameState::Multiplayer)).run_if(in_state(MultiplayerState::Disabled)),
          
        )
        .add_systems(
            Update,
            (lobby_menu_action, button_system).run_if(in_state(GameState::Multiplayer)).run_if(in_state(MultiplayerState::Lobby)),
          
        )
        .add_systems(
            OnEnter(NetworkingState::Join),
            client::create_client
        )
        .add_systems(
            Update,
            client::update.run_if(in_state(NetworkingState::Join))
        )
        .add_systems(
            OnEnter(NetworkingState::Host),
                 server::create_server
        )
        .add_systems(
            Update,
            server::update.run_if(in_state(NetworkingState::Host))
        );
    }

}

// Tag component used to tag entities added on the multiplayer  screen
#[derive(Component)]
struct OnMultiplayerScreen;
// Tag component used to tag entities added on the multiplayer  screen
#[derive(Component)]
struct OnLobbyScreen;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

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



fn multiplayer_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(250.0),
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
    
    commands.insert_resource(SocketAddress(String::new()));
    commands.insert_resource(ServerSocket(None));
    commands.insert_resource(ClientSocket(None));
    
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
            OnMultiplayerScreen,
        ))
        // .insert(InputText(String::new()))
        .with_children(|parent| { 
            parent 
                .spawn(
                    NodeBundle { 
                        style: Style {
                            width: Val::Px(250.0),
                            height: Val::Px(40.0),
                            margin: UiRect::all(Val::Px(20.0)),
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    }
                )
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "Want to join a game? Type in the host's ip address: ",
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
                    parent.spawn(TextBundle::from_section(
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
                            MultiplayerButtonAction::JoinGame,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Join Game",
                                button_text_style.clone(),
                            ));
                        });
                    });
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn(
                        TextBundle::from_section(
                            "Want to host a game? Press the button below.",
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

                    // Display three buttons for each action available from the main menu:
                    // - new game
                    // - settings
                    // - quit
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MultiplayerButtonAction::HostGame,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Host Game",
                                button_text_style.clone(),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MultiplayerButtonAction::Back,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Back",
                                button_text_style.clone(),
                            ));
                        });
                });
        });
}



fn multiplayer_menu_action(
    interaction_query: Query<
        (&Interaction, &MultiplayerButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut multiplayer_state: ResMut<NextState<MultiplayerState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut network_state: ResMut<NextState<NetworkingState>>,
) {
    for (interaction, multiplayer_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match multiplayer_button_action {
                MultiplayerButtonAction::HostGame => {
                    multiplayer_state.set(MultiplayerState::Lobby);
                    network_state.set(NetworkingState::Host);
                }
                MultiplayerButtonAction::JoinGame => 
                {
                    multiplayer_state.set(MultiplayerState::Lobby);
                    network_state.set(NetworkingState::Join);
                }
                MultiplayerButtonAction::Multiplayer => 
                {
                    multiplayer_state.set(MultiplayerState::Disabled);
                    game_state.set(GameState::Multiplayer);
                }
                MultiplayerButtonAction::Back => 
                {
                    multiplayer_state.set(MultiplayerState::Disabled);
                    game_state.set(GameState::Setup);
                }
            }
        }
    }
}

fn update_user_input(mut char_input_events: EventReader<ReceivedCharacter>, keyboard: Res<Input<KeyCode>>, mut textbox: Query<(&mut Text, &mut InputText), With<InputText>>,
mut server_address: ResMut<SocketAddress>) {

    let (mut single_box, mut user_input) = textbox.single_mut();
    for event in char_input_events.iter() {
        if keyboard.pressed(KeyCode::Back){
            single_box.sections[0].value.pop();
            user_input.0.pop();
            info!("{}", user_input.0);
        }
        else{
            single_box.sections[0].value.push(event.char);
            user_input.0.push(event.char);
            info!("{}", user_input.0);
        }
        server_address.0 = single_box.sections[0].value.clone();
    }
}

fn lobby_setup(mut commands: Commands) {
    // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(250.0),
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
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
            OnLobbyScreen,
        ))
        .with_children(|parent| { 
            parent 
                .spawn(
                    NodeBundle { 
                        style: Style {
                            width: Val::Px(250.0),
                            height: Val::Px(40.0),
                            margin: UiRect::all(Val::Px(20.0)),
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    }
                )
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "Lobby",
                            TextStyle {
                                font_size: 60.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );
                    // Display the user input
                    let user_input_text = unsafe {
                        if let Some(user_input) = &USER_INPUT {
                            user_input.clone()
                        } else {
                            String::new()
                        }
                    };
                    parent.spawn(
                        TextBundle::from_section(
                            format!("IP: {}", user_input_text),
                            TextStyle {
                                font_size: 30.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        })
                    );
                    parent.spawn(
                        TextBundle::from_section(
                            format!("Players: -"),
                            TextStyle {
                                font_size: 30.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        })
                    );
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            LobbyButtonAction::Back,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Back",
                                button_text_style.clone(),
                            ));
                        });
                });
            });
}


fn lobby_menu_action(
    interaction_query: Query<
        (&Interaction, &LobbyButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut multiplayer_state: ResMut<NextState<MultiplayerState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, lobby_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match lobby_button_action {
                LobbyButtonAction::Back => {
                    multiplayer_state.set(MultiplayerState::Main);
                }
            }
        }
    }
}