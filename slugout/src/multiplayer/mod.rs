use bevy::{app::AppExit, prelude::*, window::ReceivedCharacter};

use super::{despawn_screen, GameState, TEXT_COLOR};

pub struct MultiplayerPlugin;

#[derive(Component)]
enum MultiplayerButtonAction {
    HostGame,
    Multiplayer,
    Back,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MultiplayerState {
    Main,
    #[default]
    Disabled,
}


#[derive(Component)]
struct SelectedOption;

#[derive(Component)]
struct InputText(String);



impl Plugin for MultiplayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<MultiplayerState>()
        .add_systems(OnEnter(GameState::Multiplayer), multiplayer_setup)
        .add_systems(OnExit(GameState::Multiplayer), despawn_screen::<OnMultiplayerScreen>)
        .add_systems(
            Update,
            (multiplayer_menu_action, button_system, update_user_input).run_if(in_state(GameState::Multiplayer)),
          
        ); 
    }

}

// Tag component used to tag entities added on the multiplayer  screen
#[derive(Component)]
struct OnMultiplayerScreen;

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
    

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
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
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    }
                )
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                            String::new().to_string(),
                            TextStyle {
                                font_size: 12.0,
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
                            "Cats With Bats",
                            TextStyle {
                                font_size: 80.0,
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
) {
    for (interaction, multiplayer_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match multiplayer_button_action {
                MultiplayerButtonAction::HostGame => app_exit_events.send(AppExit), //for right now HostGame closes the game
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

fn update_user_input(mut char_input_events: EventReader<ReceivedCharacter>, keyboard: Res<Input<KeyCode>>, mut textbox: Query<(&mut Text, &mut InputText), With<InputText>>,) {

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
    }
}
