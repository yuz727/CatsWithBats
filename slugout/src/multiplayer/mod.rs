use crate::NetworkingState;
use crate::game::components::BallVelocity;
use crate::game::ball::BallNumber;
use crate::game::player_movement::PlayerVelocity;

use super::{despawn_screen, GameState, MultiplayerState, TEXT_COLOR};
use crate::multiplayer::helper::generate_server_rsa_keypair;
use bevy::{ecs::query::Has, prelude::*, window::ReceivedCharacter};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
pub struct MultiplayerPlugin;

pub mod client;
mod helper;
pub mod server;
// mod endgame;

#[derive(Event)]
pub struct ClientPlayerInfo
{
    pub data: Vec<AuthenticatedClient>
}
#[derive(Event)]
pub struct ClientBallInfo
{
    pub data: Vec<BallInfo>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerInfo {
    pub position: (f32, f32),
    pub velocity: PlayerVelocity,
    pub health: i32,
    // Add other relevant fields here
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BatInfo {
  pub is_swinging: bool,
  pub is_left: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BallInfo {
    pub position: (f32, f32),
    pub velocity: BallVelocity,
    pub ball_number: BallNumber,
}

#[derive(Component)]
enum MultiplayerButtonAction {
    HostGame,
    JoinGame,
    Back,
}

#[derive(Component)]
enum LobbyButtonAction {
    Start,
    Back,
}
#[derive(Component)]
struct SelectedOption;

#[derive(Resource)]
pub struct SocketAddress(pub String);
#[derive(Resource)]
pub struct PlayerNumber(pub u32);


// Track which stage in authentication the client is on
#[derive(Clone, PartialEq, Debug)]
pub enum ServerHandshakeStage {
    NotStarted,
    SentPublicKey,
    SentChallenge,
    Authenticated,
    FailedAuthentication,
}
#[derive(PartialEq, Clone, Debug)]
pub enum ClientHandshakeStage {
    NotStarted,
    RequestedPublicKey,
    SharedSessionKey,
    RespondedToChallenge,
    Authenticated,
    FailedAuthentication,
}

// Lets keep track of our clients

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthenticatedClient {
    pub username: String,
    pub client_address: SocketAddr,
    pub player_info: Option<PlayerInfo>,
    pub bat_info: Option<BatInfo>,
}

#[derive(Resource)]
pub struct ClientSocket {
    pub socket: Option<UdpSocket>,
    pub server_address: Option<SocketAddr>,
    pub client_address: Option<SocketAddr>,
    pub stage: ClientHandshakeStage,
    pub server_public_key: Option<RsaPublicKey>,
    pub aes_key: Option<[u8; 16]>,
}

// Struct to store where the client is at
#[derive(Resource)]
pub struct ServerSocket {
    pub socket: Option<UdpSocket>,
    pub private_key: Option<RsaPrivateKey>,
    pub public_key: Option<RsaPublicKey>,
    pub clients: HashMap<SocketAddr, ServerHandshakeStage>,
    pub client_keys: HashMap<SocketAddr, Option<[u8; 16]>>,
    pub client_challenges: HashMap<SocketAddr, u64>,
    pub authenticated_clients: Vec<AuthenticatedClient>,
    pub yarn_balls: Vec<BallInfo>,
    pub bats: Vec<BatInfo>,
}

#[derive(Resource, Serialize, Deserialize)]
pub struct ClientListVector(pub Vec<AuthenticatedClient>);

#[derive(Resource, Serialize, Deserialize)]
pub struct BallListVector(pub Vec<BallInfo>);

#[derive(Component)]
struct InputText(pub String);

#[derive(Component)]
struct ClientList;

#[derive(Serialize, Deserialize)]
struct ConnectRequest {}

#[derive(Serialize, Deserialize)]
struct ConnectResponse {
    player_number: usize,
}

pub static mut USER_INPUT: Option<String> = None;

impl Plugin for MultiplayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MultiplayerState>()
            .add_state::<NetworkingState>()
            .add_event::<ClientBallInfo>()
            .add_event::<ClientPlayerInfo>()
            .insert_resource(SocketAddress(String::new()))
            .insert_resource(ClientListVector(Vec::new()))
            .insert_resource(PlayerNumber(1))
            .insert_resource(ClientSocket {
                socket: None,
                server_address: None,
                client_address: None,
                stage: ClientHandshakeStage::NotStarted,
                server_public_key: None,
                aes_key: None,
            })
            .insert_resource(ServerSocket {
                socket: None,
                private_key: None,
                public_key: None,
                clients: HashMap::new(),
                client_keys: HashMap::new(),
                client_challenges: HashMap::new(),
                authenticated_clients: Vec::new(),
                yarn_balls: Vec::new(),
                bats: Vec::new(),
            })
            .insert_resource(BallListVector(Vec::new()))
            //.add_systems(OnEnter(GameState::Multiplayer), multiplayer_setup)
            .add_systems(OnEnter(MultiplayerState::Main), multiplayer_setup)
            .add_systems(
                OnExit(MultiplayerState::Main),
                despawn_screen::<OnMultiplayerScreen>,
            )
            .add_systems(
                OnExit(MultiplayerState::Lobby),
                despawn_screen::<OnLobbyScreen>,
            )
            .add_systems(OnEnter(NetworkingState::Host), lobby_setup_host)
            .add_systems(OnEnter(NetworkingState::Join), lobby_setup_client)
            .add_systems(
                Update,
                (multiplayer_menu_action, button_system, update_user_input)
                    .run_if(in_state(MultiplayerState::Main)),
            )
            .add_systems(
                Update,
                (lobby_menu_action, button_system)
                    .run_if(in_state(MultiplayerState::Lobby)),
            )

            // JUST CLIENT
            .add_systems(OnEnter(NetworkingState::Join), client::create_client)
            

            
            .add_systems(
                Update, 
                client::update.after(client::create_client)
                .run_if(in_state(MultiplayerState::Lobby)))
            
            .add_systems(
                Update, 
                client::update.after(client::create_client)
               
                .run_if(in_state(MultiplayerState::StartGame))
            )
            // GAME 
            // CLIENT: 
            
            // send server updated position 
            // .add_systems(
            //     Update,
            //     client::send_server_updated_position.after(crate::game::player_movement::move_player_mult)
            //     .run_if(in_state(MultiplayerState::Game))
            // )

            // receive servers updated posiitons
            .add_systems(
                Update,
                (client::update_client_data)
                .after(crate::game::setup_mult).run_if(in_state(MultiplayerState::Game)))
            
            // GAME HOST 

            // receive one clients updated positions 
            .add_systems(
                Update, 
                server::received_update.run_if(in_state(NetworkingState::Host)).run_if(in_state(MultiplayerState::Game))
            )

            // send client list to all clients 
          

            .add_systems(
                Update,
                server::update.run_if(in_state(NetworkingState::Host)).run_if(in_state(MultiplayerState::Lobby)),
            )

            // CLIENT AND HOST
            .add_systems(
                OnEnter(NetworkingState::Host),
                helper::generate_server_rsa_keypair,
            )
            .add_systems(OnEnter(NetworkingState::Host), server::create_server)
            .add_systems(
                OnEnter(NetworkingState::Host),
                client::create_client.after(server::create_server),
            )
            .add_systems(OnEnter(MultiplayerState::StartGame), server::send_start_signal.run_if(in_state(NetworkingState::Host)))
            
           
          

            .add_systems(
                Update,
                update_client_list_for_client.run_if(in_state(NetworkingState::Host)).run_if(in_state(MultiplayerState::Lobby)),
            )
            .add_systems(
                Update,
                server::send_client_list.run_if(in_state(NetworkingState::Host)).run_if(in_state(MultiplayerState::Lobby)),
            )
            .add_systems(
                Update,
                update_client_list_for_client.run_if(in_state(NetworkingState::Join)).run_if(in_state(MultiplayerState::Lobby)),
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

fn multiplayer_setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
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
            OnMultiplayerScreen,
        ))
        // .insert(InputText(String::new()))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
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
                })
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
                            parent
                                .spawn(TextBundle::from_section("Back", button_text_style.clone()));
                        });
                });
        });
}

fn multiplayer_menu_action(
    interaction_query: Query<
        (&Interaction, &MultiplayerButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
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
                MultiplayerButtonAction::JoinGame => {
                    multiplayer_state.set(MultiplayerState::Lobby);
                    network_state.set(NetworkingState::Join);
                }
                MultiplayerButtonAction::Back => {
                    multiplayer_state.set(MultiplayerState::Disabled);
                    game_state.set(GameState::Setup);
                }
            }
        }
    }
}

fn update_user_input(
    mut char_input_events: EventReader<ReceivedCharacter>,
    keyboard: Res<Input<KeyCode>>,
    mut textbox: Query<(&mut Text, &mut InputText), With<InputText>>,
    mut server_address: ResMut<SocketAddress>,
) {
    let (mut single_box, mut user_input) = textbox.single_mut();
    for event in char_input_events.iter() {
        if keyboard.pressed(KeyCode::Back) {
            single_box.sections[0].value.pop();
            user_input.0.pop();
            // info!("{}", user_input.0);
        } else {
            single_box.sections[0].value.push(event.char);
            user_input.0.push(event.char);
            // info!("{}", user_input.0);
        }
        server_address.0 = single_box.sections[0].value.clone();
    }
}

fn lobby_setup_host(mut commands: Commands, client_list: ResMut<ServerSocket>, socket_address: ResMut<SocketAddress>) {
    let address_string: String = socket_address.0.clone();
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
                .spawn(NodeBundle {
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
                })
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
                  
                  
                    parent.spawn(
                        TextBundle::from_section(
                            format!("IP: {}", address_string),
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
                    parent.spawn(
                        TextBundle::from_section(
                            format!("Players: "),
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
                    parent.spawn(
                        TextBundle::from_section(
                            String::new(),
                            TextStyle {
                                font_size: 20.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    ).insert(ClientList);
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            LobbyButtonAction::Start,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Start",
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
                            LobbyButtonAction::Back,
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_section("Back", button_text_style.clone()));
                        });
                });
        });
}

fn lobby_setup_client(mut commands: Commands, client_list: ResMut<ServerSocket>, socket_address: ResMut<SocketAddress>) {

    let address_string: String = socket_address.0.clone();
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
                .spawn(NodeBundle {
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
                })
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
                  
                    parent.spawn(
                        TextBundle::from_section(
                            format!("IP: {}", address_string),
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
                        }),
                    );
                    parent.spawn(
                        TextBundle::from_section(
                            String::new(),
                            TextStyle {
                                font_size: 20.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    ).insert(ClientList)
                    ;
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
                            parent
                                .spawn(TextBundle::from_section("Back", button_text_style.clone()));
                        });
                });
        });
}

fn lobby_menu_action(
    interaction_query: Query<
        (&Interaction, &LobbyButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut multiplayer_state: ResMut<NextState<MultiplayerState>>,
) {
    for (interaction, lobby_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match lobby_button_action {
                LobbyButtonAction::Start => {
                    // TODO: Add code for starting the game here
                    // For now, just print a message
                    println!("Starting the game!");
                    multiplayer_state.set(MultiplayerState::StartGame);
                }
                LobbyButtonAction::Back => {
                    multiplayer_state.set(MultiplayerState::Main);
                }
            }
        }
    }
}


fn update_client_list(
    mut textbox: Query<&mut Text, With<ClientList>>,
    mut server_socket: ResMut<ServerSocket>,
) {
    let mut list = textbox.single_mut();
    list.sections[0].value = String::new();
    let mut counter = 1;
    for client in server_socket.authenticated_clients.iter_mut()
    {
        // list.sections[0].value.push_str("Player ");
        // list.sections[0].value.push(char::from_digit(counter, 10).unwrap());
        // list.sections[0].value.push_str("- ");
        list.sections[0].value.push_str(client.username.as_str());
        list.sections[0].value.push('\n');
        counter = counter + 1;
    }
}
fn update_client_list_for_client(
    mut textbox: Query<&mut Text, With<ClientList>>,
    mut client_list: ResMut<ClientListVector>,
    client_socket: Res<ClientSocket>,
    mut player_num: ResMut<PlayerNumber>,
) {
    // println!("{}", client_list.0.len());
    let mut list = textbox.single_mut();
    list.sections[0].value = String::new();
    let mut counter = 1;
    for client in client_list.0.iter_mut()
    {
        // list.sections[0].value.push_str("Player ");
        // list.sections[0].value.push(char::from_digit(counter, 10).unwrap());
        // list.sections[0].value.push_str("- ");
        if client_socket.client_address.eq(&Some(client.client_address))
        {
            player_num.0 = client.username[4..client.username.len()].parse().unwrap();
        }
        list.sections[0].value.push_str(client.username.as_str());
        list.sections[0].value.push('\n');
        counter = counter + 1;
    }
}

