use bevy::{app::AppExit, prelude::*};

use super::{despawn_screen, GameState, TEXT_COLOR};

pub struct SetupPlugin;

#[derive(Component)]
enum SetupButtonAction {
    Singleplayer,
    Multiplayer,
    Back,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum SetupState {
    Main,
    #[default]
    Disabled,
}

#[derive(Component)]
struct SelectedOption;



impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<SetupState>()
        .add_systems(OnEnter(GameState::Setup), setup_state_setup)
        .add_systems(OnEnter(GameState::Setup), setup_menu_setup)
        .add_systems(OnExit(GameState::Setup), despawn_screen::<OnSetUpScreen>)
        .add_systems(
            Update,
            (setup_menu_action, button_system).run_if(in_state(GameState::Setup)),
        ); 
        
    }

}

// Tag component used to tag entities added on the setup  screen
#[derive(Component)]
struct OnSetUpScreen;

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

fn setup_state_setup(mut setup_state: ResMut<NextState<SetupState>>) {
    setup_state.set(SetupState::Main);
}

fn setup_menu_setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(275.0),
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
            OnSetUpScreen,
        ))
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
                    parent
                    .spawn((
                        ButtonBundle {
                            style: button_style.clone(),
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        },
                        SetupButtonAction::Singleplayer,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Single Player",
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
                        SetupButtonAction::Multiplayer,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Multiplayer",
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
                        SetupButtonAction::Back,
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

fn setup_menu_action(
    interaction_query: Query<
        (&Interaction, &SetupButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut _app_exit_events: EventWriter<AppExit>,
    mut setup_state: ResMut<NextState<SetupState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, setup_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match setup_button_action {
                SetupButtonAction::Singleplayer => 
                {
                    setup_state.set(SetupState::Disabled);
                    game_state.set(GameState::Game);
                } //for right now singleplayer closes the game
                SetupButtonAction::Multiplayer => 
                {
                    setup_state.set(SetupState::Disabled);
                    game_state.set(GameState::Multiplayer);
                }
                SetupButtonAction::Back => 
                {
                    setup_state.set(SetupState::Disabled);
                    game_state.set(GameState::Menu);
                }
            }
        }
    }
}