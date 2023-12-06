use std::f32::consts::PI;

use bevy::prelude::*;

use crate::multiplayer::{BallInfo, BallListVector, ClientListVector, PlayerNumber, ServerSocket, ClientBallInfo, PlayerInfo};
use crate::multiplayer::server::OtherPlayer;
use crate::{GameState, MultiplayerState, NetworkingState};
//use bevy::window::CursorMoved;

use super::components::Aim;
use super::components::Ball;
use super::components::BallVelocity;
use super::components::Bat;
use super::components::Colliding;
use super::components::Health;
use super::components::Player;
use super::components::Rug;
use crate::game::components::HealthHitbox;
use crate::game::components::Hitbox;
use crate::game::npc::NPC;
use serde::{Serialize, Deserialize};

use crate::MAP;

const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const BALL_SIZE: f32 = 10.;
//const HIT_POWER: Vec3 = Vec3::new(500.0, 500.0, 2.0);
const BASE_FRICTION: f32 = 0.4;
const G: f32 = 9.81;
const MIN_BALL_VELOCITY: f32 = 30.;
const PLAYER_SIZE: f32 = 15.;

pub struct BallPlugin;

#[derive(Component, Serialize, Deserialize, Clone, PartialEq)]
pub struct BallNumber
{
    pub number: u32
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum BallState
{
    EndSetup,
    Game,
    #[default]
    Disabled,
}

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<BallState>();
        app.add_systems(OnEnter(GameState::Game), setup);
        app.add_systems(OnEnter(MultiplayerState::Game), setup_mult.after(super::setup_mult));
        app.add_systems(Update, your_ball_player_collisions.after(super::setup_mult).after(setup_mult).run_if(in_state(MultiplayerState::Game)));
        app.add_systems(OnEnter(BallState::EndSetup), insert_balls_to_vec.after(setup_mult).
            run_if(in_state(NetworkingState::Host)));
        if unsafe { MAP } == 1 {
            app.add_systems(Update, bounce);
        } else if unsafe { MAP } == 2 {
            app.add_systems(Update, bounce_m2);
        } else if unsafe { MAP } == 3 {
            app.add_systems(Update, bounce_m2);
            app.add_systems(Update, friction_map3.run_if(in_state(GameState::Game)));
        } else if unsafe { MAP } == 4 {
            app.add_systems(Update, bounce_m2);
            app.add_systems(Update, friction_map4.run_if(in_state(GameState::Game)));
        }
        //app.add_systems(Update, bounce/* .run_if(in_state(MultiplayerState::Game)))*/;

        app.add_systems(Update, bounce_balls.after(bounce));
        if unsafe { MAP } == 4 {
            app.add_systems(Update, swing_m4.run_if(in_state(GameState::Game)));
            app.add_systems(Update, swing_m4.run_if(in_state(MultiplayerState::Game)));
        } else {
            app.add_systems(Update, swing.run_if(in_state(GameState::Game)));
            app.add_systems(Update, swing.run_if(in_state(MultiplayerState::Game)));
        }
        if unsafe { MAP } == 1 {
            app.add_systems(Update, friction_map1.run_if(in_state(GameState::Game)));
        }
        if unsafe { MAP } == 1 {
            app.add_systems(
                Update,
                friction_map1.run_if(in_state(MultiplayerState::Game)),
            );
        }
        app.add_systems(Update, bat_hitbox.run_if(in_state(GameState::Game)));
        app.add_systems(Update, bat_hitbox_mult.run_if(in_state(MultiplayerState::Game)));
        app.add_systems(Update, aim_follows_cursor.run_if(in_state(GameState::Game)));
        app.add_systems(
            Update,
            ball_npc_collisions.run_if(in_state(GameState::Game)),
        );
        app.add_systems(
            Update,
            ball_player_collisions.run_if(in_state(GameState::Game)),
        );
        app.add_systems(Update, update_balls.run_if(in_state(MultiplayerState::Game)));
        app.add_systems(
            Update,
            aim_follows_cursor.run_if(in_state(MultiplayerState::Game)),
        )
        .insert_resource(Input::<KeyCode>::default());
    }
}

//ball Creation
//for map 2,3,4 change velocity to 0 or reduced
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    if unsafe { MAP == 1 || MAP == 4 } {
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("yarnball.png"),
                transform: Transform::from_xyz(100., 100., 2.)
                    .with_scale(Vec3::new(0.025, 0.025, 0.)),
                ..Default::default()
            })
            .insert(Ball {
                radius: 2.5,
                elasticity: 0.95,
                prev_pos: Vec3::splat(0.),
                density: 2.,
            })
            .insert(BallVelocity {
                velocity: Vec3::new(300.0, 300.0, 2.0),
            })
            .insert(Colliding::new());

        // 2ND ball
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("yarnball.png"),
                transform: Transform::from_xyz(300., 300., 2.)
                    .with_scale(Vec3::new(0.028, 0.028, 0.)),
                ..Default::default()
            })
            .insert(Ball {
                radius: 2.8,
                elasticity: 1.,
                prev_pos: Vec3::splat(0.),
                density: 4.,
            })
            .insert(BallVelocity {
                velocity: Vec3::new(300.0, 100.0, 2.0),
            })
            .insert(Colliding::new());

        //3RD ball
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("yarnball.png"),
                transform: Transform::from_xyz(300., 200., 2.)
                    .with_scale(Vec3::new(0.031, 0.031, 0.)),
                ..Default::default()
            })
            .insert(Ball {
                radius: 3.1,
                elasticity: 0.975,
                prev_pos: Vec3::splat(0.),
                density: 6.,
            })
            .insert(BallVelocity {
                velocity: Vec3::new(-500., 3., 2.),
            })
            .insert(Colliding::new());

        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("yarnball.png"),
                transform: Transform::from_xyz(100., 105., 2.)
                    .with_scale(Vec3::new(0.034, 0.034, 0.)),
                ..Default::default()
            })
            .insert(Ball {
                radius: 3.4,
                elasticity: 0.9,
                prev_pos: Vec3::splat(0.),
                density: 8.,
            })
            .insert(BallVelocity {
                velocity: Vec3::new(300.0, 300.0, 2.0),
            })
            .insert(Colliding::new());
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("yarnball.png"),
                transform: Transform::from_xyz(-200., 300., 2.)
                    .with_scale(Vec3::new(0.038, 0.038, 0.)),
                ..Default::default()
            })
            .insert(Ball {
                radius: 3.8,
                elasticity: 0.875,
                prev_pos: Vec3::splat(0.),
                density: 10.,
            })
            .insert(BallVelocity {
                velocity: Vec3::new(300.0, 300.0, 2.0),
            })
            .insert(Colliding::new());
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("yarnball.png"),
                transform: Transform::from_xyz(450., 250., 2.)
                    .with_scale(Vec3::new(0.042, 0.042, 0.)),
                ..Default::default()
            })
            .insert(Ball {
                radius: 4.2,
                elasticity: 0.85,
                prev_pos: Vec3::splat(0.),
                density: 3.,
            })
            .insert(BallVelocity {
                velocity: Vec3::new(300.0, 300.0, 2.0),
            })
            .insert(Colliding::new());
    } else if unsafe { MAP == 3 } {
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("yarnball.png"),
                transform: Transform::from_xyz(100., 100., 2.)
                    .with_scale(Vec3::new(0.025, 0.025, 0.)),
                ..Default::default()
            })
            .insert(Ball {
                radius: 2.5,
                elasticity: 0.95,
                prev_pos: Vec3::splat(0.),
                density: 2.,
            })
            .insert(BallVelocity {
                velocity: Vec3::new(600.0, 600.0, 2.0),
            })
            .insert(Colliding::new());

        // 2ND ball
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("yarnball.png"),
                transform: Transform::from_xyz(300., 300., 2.)
                    .with_scale(Vec3::new(0.028, 0.028, 0.)),
                ..Default::default()
            })
            .insert(Ball {
                radius: 2.8,
                elasticity: 1.,
                prev_pos: Vec3::splat(0.),
                density: 4.,
            })
            .insert(BallVelocity {
                velocity: Vec3::new(600.0, 200.0, 2.0),
            })
            .insert(Colliding::new());

        //3RD ball
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("yarnball.png"),
                transform: Transform::from_xyz(300., 200., 2.)
                    .with_scale(Vec3::new(0.031, 0.031, 0.)),
                ..Default::default()
            })
            .insert(Ball {
                radius: 3.1,
                elasticity: 0.975,
                prev_pos: Vec3::splat(0.),
                density: 6.,
            })
            .insert(BallVelocity {
                velocity: Vec3::new(-1000., 6., 2.),
            })
            .insert(Colliding::new());

        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("yarnball.png"),
                transform: Transform::from_xyz(100., 105., 2.)
                    .with_scale(Vec3::new(0.034, 0.034, 0.)),
                ..Default::default()
            })
            .insert(Ball {
                radius: 3.4,
                elasticity: 0.9,
                prev_pos: Vec3::splat(0.),
                density: 8.,
            })
            .insert(BallVelocity {
                velocity: Vec3::new(600.0, 600.0, 2.0),
            })
            .insert(Colliding::new());
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("yarnball.png"),
                transform: Transform::from_xyz(-200., 300., 2.)
                    .with_scale(Vec3::new(0.038, 0.038, 0.)),
                ..Default::default()
            })
            .insert(Ball {
                radius: 3.8,
                elasticity: 0.875,
                prev_pos: Vec3::splat(0.),
                density: 10.,
            })
            .insert(BallVelocity {
                velocity: Vec3::new(600.0, 600.0, 2.0),
            })
            .insert(Colliding::new());
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("yarnball.png"),
                transform: Transform::from_xyz(450., 50., 2.)
                    .with_scale(Vec3::new(0.042, 0.042, 0.)),
                ..Default::default()
            })
            .insert(Ball {
                radius: 4.2,
                elasticity: 0.85,
                prev_pos: Vec3::splat(0.),
                density: 3.,
            })
            .insert(BallVelocity {
                velocity: Vec3::new(600.0, 600.0, 2.0),
            })
            .insert(Colliding::new());
    } else if unsafe { MAP == 2 } {
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("yarnball.png"),
                transform: Transform::from_xyz(-100., 100., 2.)
                    .with_scale(Vec3::new(0.025, 0.025, 0.)),
                ..Default::default()
            })
            .insert(Ball {
                radius: 2.5,
                elasticity: 0.95,
                prev_pos: Vec3::splat(0.),
                density: 2.,
            })
            .insert(BallVelocity {
                velocity: Vec3::new(0.0, 0.0, 2.0),
            })
            .insert(Colliding::new());

        // 2ND ball
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("yarnball.png"),
                transform: Transform::from_xyz(300., -200., 2.)
                    .with_scale(Vec3::new(0.028, 0.028, 0.)),
                ..Default::default()
            })
            .insert(Ball {
                radius: 2.8,
                elasticity: 1.,
                prev_pos: Vec3::splat(0.),
                density: 4.,
            })
            .insert(BallVelocity {
                velocity: Vec3::new(0.0, 0.0, 2.0),
            })
            .insert(Colliding::new());

        //3RD ball
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("yarnball.png"),
                transform: Transform::from_xyz(300., 300., 2.)
                    .with_scale(Vec3::new(0.031, 0.031, 0.)),
                ..Default::default()
            })
            .insert(Ball {
                radius: 3.1,
                elasticity: 0.975,
                prev_pos: Vec3::splat(0.),
                density: 6.,
            })
            .insert(BallVelocity {
                velocity: Vec3::new(0.0, 0.0, 2.0),
            })
            .insert(Colliding::new());

        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("yarnball.png"),
                transform: Transform::from_xyz(100., 105., 2.)
                    .with_scale(Vec3::new(0.034, 0.034, 0.)),
                ..Default::default()
            })
            .insert(Ball {
                radius: 3.4,
                elasticity: 0.9,
                prev_pos: Vec3::splat(0.),
                density: 8.,
            })
            .insert(BallVelocity {
                velocity: Vec3::new(0.0, 0.0, 2.0),
            })
            .insert(Colliding::new());
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("yarnball.png"),
                transform: Transform::from_xyz(-200., 300., 2.)
                    .with_scale(Vec3::new(0.038, 0.038, 0.)),
                ..Default::default()
            })
            .insert(Ball {
                radius: 3.8,
                elasticity: 0.875,
                prev_pos: Vec3::splat(0.),
                density: 10.,
            })
            .insert(BallVelocity {
                velocity: Vec3::new(0.0, 0.0, 2.0),
            })
            .insert(Colliding::new());
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("yarnball.png"),
                transform: Transform::from_xyz(-350., -200., 2.)
                    .with_scale(Vec3::new(0.042, 0.042, 0.)),
                ..Default::default()
            })
            .insert(Ball {
                radius: 4.2,
                elasticity: 0.85,
                prev_pos: Vec3::splat(0.),
                density: 3.,
            })
            .insert(BallVelocity {
                velocity: Vec3::new(0.0, 0.0, 2.0),
            })
            .insert(Colliding::new());
    }
    /*// added for debugging

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("yarnball.png"),
            transform: Transform::from_xyz(7., 400., 2.).with_scale(Vec3::new(0.025, 0.025, 0.)),
            ..Default::default()
        })
        .insert(Ball {
            radius: 2.5,
            elasticity: 0.95,
            prev_pos: Vec3::splat(0.),
            density: 7.,
        })
        .insert(BallVelocity {
            velocity: Vec3::new(67.0, 282.0, 2.0),
        })
        .insert(Colliding::new());
    // 2ND ball
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("yarnball.png"),
            transform: Transform::from_xyz(360., 52., 2.).with_scale(Vec3::new(0.028, 0.028, 0.)),
            ..Default::default()
        })
        .insert(Ball {
            radius: 2.8,
            elasticity: 1.,
            prev_pos: Vec3::splat(0.),
            density: 9.,
        })
        .insert(super::components::BallVelocity {
            velocity: Vec3::new(300.0, 100.0, 2.0),
        })
        .insert(Colliding::new());
    //3RD ball
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("yarnball.png"),
            transform: Transform::from_xyz(-400., 0., 2.)
                .with_scale(Vec3::new(0.031, 0.031, 0.)),
            ..Default::default()
        })
        .insert(Ball {
            radius: 3.1,
            elasticity: 0.975,
            prev_pos: Vec3::splat(0.),
            density: 2.,
        })
        .insert(BallVelocity {
            velocity: Vec3::new(-500., 3., 2.),
        })
        .insert(Colliding::new());
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("yarnball.png"),
            transform: Transform::from_xyz(82., 26., 2.).with_scale(Vec3::new(0.034, 0.034, 0.)),
            ..Default::default()
        })
        .insert(Ball {
            radius: 3.4,
            elasticity: 0.9,
            prev_pos: Vec3::splat(0.),
            density: 1.,
        })
        .insert(BallVelocity {
            velocity: Vec3::new(70.0, 300.0, 2.0),
        })
        .insert(Colliding::new());
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("yarnball.png"),
            transform: Transform::from_xyz(38., 102., 2.).with_scale(Vec3::new(0.038, 0.038, 0.)),
            ..Default::default()
        })
        .insert(Ball {
            radius: 3.8,
            elasticity: 0.875,
            prev_pos: Vec3::splat(0.),
            density: 4.,
        })
        .insert(BallVelocity {
            velocity: Vec3::new(300.0, 300.0, 2.0),
        })
        .insert(Colliding::new());
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("yarnball.png"),
            transform: Transform::from_xyz(500., 500., 2.).with_scale(Vec3::new(0.042, 0.042, 0.)),
            ..Default::default()
        })
        .insert(Ball {
            radius: 4.2,
            elasticity: 0.85,
            prev_pos: Vec3::splat(0.),
            density: 6.,
        })
        .insert(BallVelocity {
            velocity: Vec3::new(300.0, 300.0, 2.0),
        })
        .insert(Colliding::new()); */

    //Spawn bat hitbox for bat
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(240., 140., 100., 0.),
                custom_size: Some(Vec2::new(45., 75.)),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 2.),
            ..Default::default()
        })
        .insert(Hitbox {
            size: Vec2::new(45., 75.), //30 52
        });
    //Spawn health hitbox for player
    /*commands
    .spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(240., 140., 100., 0.2),
            custom_size: Some(Vec2::new(30., 35.)),
            ..default()
        },
        transform: Transform::from_xyz(0., 0., 2.),
        visibility: Visibility::Visible,
        ..Default::default()
    })
    .insert(HealthHitbox {
        size: Vec2::new(30., 35.), //30 52
    });*/

    // Player 1 health
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Dead.png"),
            transform: Transform::with_scale(
                Transform::from_xyz(525., 280., 2.),
                Vec3::splat(0.11),
            ),
            ..default()
        })
        .insert(Health {
            lives: -1,
            player_type: "player".to_string(),
        });

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("1Health.png"),
            transform: Transform::with_scale(
                Transform::from_xyz(525., 280., 3.),
                Vec3::splat(0.11),
            ),
            ..default()
        })
        .insert(Health {
            lives: 1,
            player_type: "player".to_string(),
        });

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("2Health.png"),
            transform: Transform::with_scale(
                Transform::from_xyz(525., 280., 4.),
                Vec3::splat(0.11),
            ),
            ..default()
        })
        .insert(Health {
            lives: 2,
            player_type: "player".to_string(),
        });

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("FullHealth.png"),
            transform: Transform::with_scale(
                Transform::from_xyz(525., 280., 5.),
                Vec3::splat(0.11),
            ),
            ..default()
        })
        .insert(Health {
            lives: 3,
            player_type: "player".to_string(),
        });

    //NPC player health
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("deadNPC.png"),
            transform: Transform::with_scale(
                Transform::from_xyz(-525., 280., 2.),
                Vec3::splat(0.11),
            ),
            ..default()
        })
        .insert(Health {
            lives: -1,
            player_type: "npc".to_string(),
        });

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("1healthNPC.png"),
            transform: Transform::with_scale(
                Transform::from_xyz(-525., 280., 3.),
                Vec3::splat(0.11),
            ),
            ..default()
        })
        .insert(Health {
            lives: 1,
            player_type: "npc".to_string(),
        });

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("2healthNPC.png"),
            transform: Transform::with_scale(
                Transform::from_xyz(-525., 280., 4.),
                Vec3::splat(0.11),
            ),
            ..default()
        })
        .insert(Health {
            lives: 2,
            player_type: "npc".to_string(),
        });

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("FullHealthNPC.png"),
            transform: Transform::with_scale(
                Transform::from_xyz(-525., 280., 5.),
                Vec3::splat(0.11),
            ),
            ..default()
        })
        .insert(Health {
            lives: 3,
            player_type: "npc".to_string(),
        });
}

fn setup_mult(
    mut commands: Commands, 
    mut ball_list: ResMut<BallListVector>,
    asset_server: Res<AssetServer>, 
    client_list: Res<ClientListVector>,
    player_num: Res<PlayerNumber>,
    mut ball_state: ResMut<NextState<BallState>>,) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("yarnball.png"),
            transform: Transform::from_xyz(0., 0., 2.).with_scale(Vec3::new(0.025, 0.025, 0.)),
            ..Default::default()
        })
        .insert(Ball {
            radius: 2.5,
            elasticity: 0.95,
            prev_pos: Vec3::splat(0.),
            density: 2.,
        })
        .insert(BallVelocity {
            velocity: Vec3::new(300.0, 300.0, 2.0),
        })
        .insert(Colliding::new())
        .insert(BallNumber{
            number: 1,
        });
    ball_list.0.push(BallInfo{
        position: (0., 0.),
        velocity: BallVelocity {
            velocity: Vec3::new(300.0, 300.0, 2.0),
        },
        ball_number: BallNumber{
            number: 1,
        }
    });

    // 2ND ball
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("yarnball.png"),
            transform: Transform::from_xyz(200., 5., 2.).with_scale(Vec3::new(0.028, 0.028, 0.)),
            ..Default::default()
        })
        .insert(Ball {
            radius: 2.8,
            elasticity: 1.,
            prev_pos: Vec3::splat(0.),
            density: 4.,
        })
        .insert(BallVelocity {
            velocity: Vec3::new(300.0, 100.0, 2.0),
        })
        .insert(Colliding::new())
        .insert(BallNumber{
            number: 2,
        });

        ball_list.0.push(BallInfo{
            position: (250., 5.),
            velocity: BallVelocity {
                velocity: Vec3::new(300.0, 100.0, 2.0),
            },
            ball_number: BallNumber { number: 2 }
        });

    // //3RD ball
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("yarnball.png"),
            transform: Transform::from_xyz(-350., -100., 2.)
                .with_scale(Vec3::new(0.031, 0.031, 0.)),
            ..Default::default()
        })
        .insert(Ball {
            radius: 3.1,
            elasticity: 0.975,
            prev_pos: Vec3::splat(0.),
            density: 6.,
        })
        .insert(BallVelocity {
            velocity: Vec3::new(-500., 3., 2.),
        })
        .insert(Colliding::new())
        .insert(BallNumber{
            number: 3,
        });
        ball_list.0.push(BallInfo{
            position: (-350., -100.),
            velocity: BallVelocity {
                velocity: Vec3::new(-500.0, 3.0, 2.0),
            },
            ball_number: BallNumber { number: 3 }
        });

  
    /*// added for debugging

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("yarnball.png"),
            transform: Transform::from_xyz(7., 400., 2.).with_scale(Vec3::new(0.025, 0.025, 0.)),
            ..Default::default()
        })
        .insert(Ball {
            radius: 2.5,
            elasticity: 0.95,
            prev_pos: Vec3::splat(0.),
            density: 7.,
        })
        .insert(BallVelocity {
            velocity: Vec3::new(67.0, 282.0, 2.0),
        })
        .insert(Colliding::new());
    // 2ND ball
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("yarnball.png"),
            transform: Transform::from_xyz(360., 52., 2.).with_scale(Vec3::new(0.028, 0.028, 0.)),
            ..Default::default()
        })
        .insert(Ball {
            radius: 2.8,
            elasticity: 1.,
            prev_pos: Vec3::splat(0.),
            density: 9.,
        })
        .insert(super::components::BallVelocity {
            velocity: Vec3::new(300.0, 100.0, 2.0),
        })
        .insert(Colliding::new());
    //3RD ball
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("yarnball.png"),
            transform: Transform::from_xyz(-400., 0., 2.)
                .with_scale(Vec3::new(0.031, 0.031, 0.)),
            ..Default::default()
        })
        .insert(Ball {
            radius: 3.1,
            elasticity: 0.975,
            prev_pos: Vec3::splat(0.),
            density: 2.,
        })
        .insert(BallVelocity {
            velocity: Vec3::new(-500., 3., 2.),
        })
        .insert(Colliding::new());
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("yarnball.png"),
            transform: Transform::from_xyz(82., 26., 2.).with_scale(Vec3::new(0.034, 0.034, 0.)),
            ..Default::default()
        })
        .insert(Ball {
            radius: 3.4,
            elasticity: 0.9,
            prev_pos: Vec3::splat(0.),
            density: 1.,
        })
        .insert(BallVelocity {
            velocity: Vec3::new(70.0, 300.0, 2.0),
        })
        .insert(Colliding::new());
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("yarnball.png"),
            transform: Transform::from_xyz(38., 102., 2.).with_scale(Vec3::new(0.038, 0.038, 0.)),
            ..Default::default()
        })
        .insert(Ball {
            radius: 3.8,
            elasticity: 0.875,
            prev_pos: Vec3::splat(0.),
            density: 4.,
        })
        .insert(BallVelocity {
            velocity: Vec3::new(300.0, 300.0, 2.0),
        })
        .insert(Colliding::new());
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("yarnball.png"),
            transform: Transform::from_xyz(500., 500., 2.).with_scale(Vec3::new(0.042, 0.042, 0.)),
            ..Default::default()
        })
        .insert(Ball {
            radius: 4.2,
            elasticity: 0.85,
            prev_pos: Vec3::splat(0.),
            density: 6.,
        })
        .insert(BallVelocity {
            velocity: Vec3::new(300.0, 300.0, 2.0),
        })
        .insert(Colliding::new()); */

    //Spawn bat hitbox for bat
    for client in client_list.0.iter()
    {
        if player_num.0 == client.username[4..client.username.len()].parse::<u32>().unwrap()
        {
            commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(240., 140., 100., 0.),
                    custom_size: Some(Vec2::new(45., 75.)),
                    ..default()
                },
                transform: Transform::from_xyz(client.player_info.as_ref().unwrap().position.0, client.player_info.as_ref().unwrap().position.0, 2.),
                ..Default::default()
            })
            .insert(Hitbox {
                size: Vec2::new(45., 75.), //30 52
            })
            .insert(Player
            {
                powerup: String::new(),
                powerup_timer: 0.,
                health: 3,
                health_timer: 0.,
            });
        }
        else
        {
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(240., 140., 100., 0.),
                        custom_size: Some(Vec2::new(45., 75.)),
                        ..default()
                    },
                    transform: Transform::from_xyz(client.player_info.as_ref().unwrap().position.0, client.player_info.as_ref().unwrap().position.0, 2.),
                    ..Default::default()
                })
                .insert(Hitbox {
                    size: Vec2::new(45., 75.), //30 52
                })
                .insert(OtherPlayer);
        }
    }
    

    //Spawn health hitbox for player
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(240., 140., 100., 0.2),
                custom_size: Some(Vec2::new(30., 35.)),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 2.),
            visibility: Visibility::Hidden,
            ..Default::default()
        })
        .insert(HealthHitbox {
            size: Vec2::new(30., 35.), //30 52
        });

    // Player 1 health
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Dead.png"),
            transform: Transform::with_scale(
                Transform::from_xyz(525., 280., 2.),
                Vec3::splat(0.11),
            ),
            ..default()
        })
        .insert(Health
        {
            lives: 0,
            player_type: String::new(),
        });

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("1Health.png"),
            transform: Transform::with_scale(
                Transform::from_xyz(525., 280., 3.),
                Vec3::splat(0.11),
            ),
            ..default()
        })
        .insert(Health
            {
                lives: 1,
                player_type: String::new(),
            });

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("2Health.png"),
            transform: Transform::with_scale(
                Transform::from_xyz(525., 280., 4.),
                Vec3::splat(0.11),
            ),
            ..default()
        })
        .insert(Health
            {
                lives: 2,
                player_type: String::new(),
            });

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("FullHealth.png"),
            transform: Transform::with_scale(
                Transform::from_xyz(525., 280., 5.),
                Vec3::splat(0.11),
            ),
            ..default()
        })
        .insert(Health
            {
                lives: 3,
                player_type: String::new(),
            });
    ball_state.set(BallState::EndSetup);
}

fn insert_balls_to_vec(
    mut server_socket: ResMut<ServerSocket>,
    mut query: Query<
    (&mut Transform, &mut BallVelocity, &mut BallNumber)>,
    mut ball_state: ResMut<NextState<BallState>>,
)
{
    println!("putting balls in the vec");
    for (mut transform, mut velocity,  mut number) in query.iter_mut()
    {
        let ball = BallInfo{
            position: (transform.translation.x, transform.translation.y),
            velocity: velocity.clone(),
            ball_number: number.clone()
        };
        server_socket.yarn_balls.push(ball);
    }
        ball_state.set(BallState::Game);
    
}

//bounce the ball
pub fn bounce(
    mut client_info: ResMut<crate::multiplayer::ClientSocket>,
    ball_list: Res<BallListVector>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut BallVelocity, &mut Ball, &BallNumber), (With<Ball>, Without<Player>)>,
    //health hitbox
    mut player_query: Query<(&mut Transform, &mut Player), (With<Player>, Without<Ball>)>,
    mut healthbar_query: Query<
        (&mut Visibility, &mut Health),
        (With<Health>, Without<Ball>, Without<Player>),
    >,
    input_mouse: Res<Input<MouseButton>>,
) {
    for (mut transform, mut ball_velocity, mut ball, ball_number) in query.iter_mut() {
        //ball radius on screen
        let ball_radius = ball.radius * 3.;
        let mut collided_with = false; 
        // Find the new translation for the x and y for the ball
        let mut new_translation_x = (transform.translation.x
            + (ball_velocity.velocity.x * time.delta_seconds()))
        .clamp(-(1280. / 2.) + ball_radius, 1280. / 2. - ball_radius);

        let mut new_translation_y = (transform.translation.y
            + (ball_velocity.velocity.y * time.delta_seconds()))
        .clamp(-(720. / 2.) + ball_radius, 720. / 2. - ball_radius);

        let new_translation = Vec3::new(
            new_translation_x,
            new_translation_y,
            transform.translation.z,
        );

        let recliner_size = Vec2::new(100., 180.);
        let recliner_translation = Vec3::new(-60., 210., 1.);
        let recliner = bevy::sprite::collide_aabb::collide(
            recliner_translation,
            recliner_size,
            new_translation,
            Vec2::new(ball_radius * 2., ball_radius * 2.),
        );

        let tv_size = Vec2::new(164., 103.);
        let tv_translation = Vec3::new(0., -250., 1.);
        let tv_stand = bevy::sprite::collide_aabb::collide(
            tv_translation,
            tv_size,
            new_translation,
            Vec2::new(ball_radius * 2., ball_radius * 2.),
        );

        let table_size = Vec2::new(103., 103.);
        let table_translation = Vec3::new(120., 170., 1.);
        let side_table = bevy::sprite::collide_aabb::collide(
            table_translation,
            table_size,
            new_translation,
            Vec2::new(ball_radius * 2., ball_radius * 2.),
        );

        //other collisions//////////////////////////////////////////////////////

        if recliner == Some(bevy::sprite::collide_aabb::Collision::Right) {
            ball_velocity.velocity.x = -ball_velocity.velocity.x * 0.8 * ball.elasticity;
            ball_velocity.velocity.y = ball_velocity.velocity.y * 0.8 * ball.elasticity;
            new_translation_x = recliner_translation.x - recliner_size.x / 2. - ball_radius;
            collided_with = true;
        } else if recliner == Some(bevy::sprite::collide_aabb::Collision::Left) {
            ball_velocity.velocity.x = -ball_velocity.velocity.x * 0.8 * ball.elasticity;
            ball_velocity.velocity.y = ball_velocity.velocity.y * 0.8 * ball.elasticity;
            new_translation_x = recliner_translation.x + recliner_size.x / 2. + ball_radius;
            collided_with = true;
        } else if recliner == Some(bevy::sprite::collide_aabb::Collision::Top) {
            ball_velocity.velocity.y = -ball_velocity.velocity.y * 0.8 * ball.elasticity;
            ball_velocity.velocity.x = ball_velocity.velocity.x * 0.8 * ball.elasticity;
            new_translation_y = recliner_translation.y - recliner_size.y / 2. - ball_radius;
            collided_with = true;
        } else if recliner == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
            ball_velocity.velocity.y = -ball_velocity.velocity.y * 0.8 * ball.elasticity;
            ball_velocity.velocity.x = ball_velocity.velocity.x * 0.8 * ball.elasticity;
            new_translation_y = recliner_translation.y + recliner_size.y / 2. + ball_radius;
            collided_with = true;
        }

        if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Left) {
            ball_velocity.velocity.x = -ball_velocity.velocity.x * 0.9 * ball.elasticity;
            ball_velocity.velocity.y = ball_velocity.velocity.y * 0.9 * ball.elasticity;
            new_translation_x = tv_translation.x + tv_size.x / 2. + ball_radius;
            collided_with = true;
        } else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Right) {
            ball_velocity.velocity.x = -ball_velocity.velocity.x * 0.9 * ball.elasticity;
            ball_velocity.velocity.y = ball_velocity.velocity.y * 0.9 * ball.elasticity;
            new_translation_x = tv_translation.x - tv_size.x / 2. - ball_radius;
            collided_with = true;
        } else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Top) {
            ball_velocity.velocity.y = -ball_velocity.velocity.y * 0.9 * ball.elasticity;
            ball_velocity.velocity.x = ball_velocity.velocity.x * 0.9 * ball.elasticity;
            new_translation_y = tv_translation.y - tv_size.y / 2. - ball_radius;
            collided_with = true;
        } else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
            ball_velocity.velocity.y = -ball_velocity.velocity.y * 0.9 * ball.elasticity;
            ball_velocity.velocity.x = ball_velocity.velocity.x * 0.9 * ball.elasticity;
            new_translation_y = tv_translation.y + tv_size.y / 2. + ball_radius;
            collided_with = true;
        }

        if side_table == Some(bevy::sprite::collide_aabb::Collision::Left) {
            ball_velocity.velocity.x = -ball_velocity.velocity.x * 0.85 * ball.elasticity;
            ball_velocity.velocity.y = ball_velocity.velocity.y * 0.85 * ball.elasticity;
            new_translation_x = table_translation.x + table_size.x / 2. + ball_radius;
            collided_with = true;
        } else if side_table == Some(bevy::sprite::collide_aabb::Collision::Right) {
            ball_velocity.velocity.x = -ball_velocity.velocity.x * 0.85 * ball.elasticity;
            ball_velocity.velocity.y = ball_velocity.velocity.y * 0.85 * ball.elasticity;
            new_translation_x = table_translation.x - table_size.x / 2. - ball_radius;
            collided_with = true;
        } else if side_table == Some(bevy::sprite::collide_aabb::Collision::Top) {
            ball_velocity.velocity.y = -ball_velocity.velocity.y * 0.85 * ball.elasticity;
            ball_velocity.velocity.x = ball_velocity.velocity.x * 0.85 * ball.elasticity;
            new_translation_y = table_translation.y - table_size.y / 2. - ball_radius;
            collided_with = true;
        } else if side_table == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
            ball_velocity.velocity.y = -ball_velocity.velocity.y * 0.85 * ball.elasticity;
            ball_velocity.velocity.x = ball_velocity.velocity.x * 0.85 * ball.elasticity;
            new_translation_y = table_translation.y + table_size.y / 2. + ball_radius;
            collided_with = true;
        }

        ball.prev_pos = transform.translation;

        // Move ball
        transform.translation.x = new_translation_x;
        transform.translation.y = new_translation_y;

        // Bounce when hitting the screen edges
        if transform.translation.x.abs() == WIN_W / 2.0 - ball_radius {
            ball_velocity.velocity.x = -ball_velocity.velocity.x * ball.elasticity;
            ball_velocity.velocity.y = ball_velocity.velocity.y * ball.elasticity;
            collided_with = true;
        }
        if transform.translation.y.abs() == WIN_H / 2.0 - ball_radius {
            ball_velocity.velocity.y = -ball_velocity.velocity.y * ball.elasticity;
            ball_velocity.velocity.x = ball_velocity.velocity.x * ball.elasticity;
            collided_with = true;
        }
        if collided_with {
            for ball_check in ball_list.0.iter()
            {
                if (transform.translation.x !=  ball_check.position.0 ||
                    transform.translation.y != ball_check.position.1) &&
                    ball_number.number == ball_check.ball_number.number
                {
                    if client_info.socket.is_none() {
                        return;
                    }
                    let ball_info = BallInfo{
                        position: (transform.translation.x, transform.translation.y),
                        velocity: ball_velocity.clone(),
                        ball_number: ball_number.clone(),
                    };
                    // println!("sending ball info");
                    let socket = client_info.socket.as_mut().unwrap();
                    socket.set_nonblocking(true).expect("could not set non-blocking");
                    
                        let message = serde_json::to_string(&ball_info).expect("Failed to serialize");
            
                        let id = "BALFC";
                        let big_message = id.to_string()  + &message;
                        // println!("Bounce Sending my new ball data to the server, it is: {:?}", message);
                        // println!("what is server address string {:?}", server_address_str);
                        match socket.send(big_message.as_bytes()) {
                            Ok(_) => {},
                            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                // Ignore WouldBlock errors
                            },
                            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                                println!("Socket is already connected");
                            },
                            Err(e) => {
                                println!("failed to send server updated position");
                                println!("Failed to send data: {:?}", e);
                            }
                        }
                } 
            }
        }
       
    }
}


pub fn your_ball_player_collisions (
    mut query: Query<(&mut Transform, &mut BallVelocity, &mut Ball), (With<Ball>, Without<Player>)>,
    mut player_query: Query<(&mut Transform, &mut Player), (With<Player>, Without<Bat>, Without<Ball>)>,
    mut healthbar_query: Query<
        (&mut Visibility, &mut Health),
        (With<Health>, Without<Ball>, Without<Player>),
    >,
    input_mouse: Res<Input<MouseButton>>,
    player_num: Res<PlayerNumber>,
    time: Res<Time>,
    mut client_info: ResMut<crate::multiplayer::ClientSocket>
    , client_list: Res<ClientListVector>,
) {
    for (mut transform, mut ball_velocity, mut ball) in query.iter_mut() {
        let ball_radius = ball.radius * 3.;

        for (mut player_transform, mut player) in player_query.iter_mut() {
            let player_collision = bevy::sprite::collide_aabb::collide(
                player_transform.translation,
                Vec2::splat(PLAYER_SIZE),
                transform.translation,
                Vec2::new(ball_radius * 2., ball_radius * 2.),
            );
            let mut did_lose_health = false; 
            if player_collision == Some(bevy::sprite::collide_aabb::Collision::Right)
                && player.health > 0
            {
                ball_velocity.velocity.x = -ball_velocity.velocity.x * ball.elasticity;
                ball_velocity.velocity.y = ball_velocity.velocity.y * ball.elasticity;
                transform.translation.x = transform.translation.x - 10.;
                if !input_mouse.pressed(MouseButton::Left) && player.health_timer <= 0. {
                    for (mut health_visibility, mut healthbar) in healthbar_query.iter_mut() {
                        if healthbar.lives == player.health
                            && healthbar.player_type == "player".to_string()
                        {
                            *health_visibility = Visibility::Hidden;
                        }
                    }
                    player.health = player.health - 1;
                    player.health_timer = 10.;
                    did_lose_health = true; 
                }
            } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Left)
                && player.health > 0
            {
                ball_velocity.velocity.x = -ball_velocity.velocity.x * ball.elasticity;
                ball_velocity.velocity.y = ball_velocity.velocity.y * ball.elasticity;
                transform.translation.x = transform.translation.x + 10.;
                if !input_mouse.pressed(MouseButton::Left) && player.health_timer <= 0. {
                    for (mut health_visibility, mut healthbar) in healthbar_query.iter_mut() {
                        if healthbar.lives == player.health
                            && healthbar.player_type == "player".to_string()
                        {
                            *health_visibility = Visibility::Hidden;
                        }
                    }
                    player.health = player.health - 1;
                    player.health_timer = 10.;
                    did_lose_health = true; 
                }
            } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Top)
                && player.health > 0
            {
                ball_velocity.velocity.y = -ball_velocity.velocity.y * ball.elasticity;
                ball_velocity.velocity.x = ball_velocity.velocity.x * ball.elasticity;
                transform.translation.y = transform.translation.y - 10.;
                if !input_mouse.pressed(MouseButton::Left) && player.health_timer <= 0. {
                    for (mut health_visibility, mut healthbar) in healthbar_query.iter_mut() {
                        if healthbar.lives == player.health
                            && healthbar.player_type == "player".to_string()
                        {
                            *health_visibility = Visibility::Hidden;
                        }
                    }
                    player.health = player.health - 1;
                    player.health_timer = 10.;
                    did_lose_health = true; 
                }
            } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Bottom)
                && player.health > 0
            {
                ball_velocity.velocity.y = -ball_velocity.velocity.y * ball.elasticity;
                ball_velocity.velocity.x = ball_velocity.velocity.x * ball.elasticity;
                transform.translation.y = transform.translation.y + 10.;
                if !input_mouse.pressed(MouseButton::Left) && player.health_timer <= 0. {
                    for (mut health_visibility, mut healthbar) in healthbar_query.iter_mut() {
                        if healthbar.lives == player.health
                            && healthbar.player_type == "player".to_string()
                        {
                            *health_visibility = Visibility::Hidden;
                        }
                    }
                    player.health = player.health - 1;
                    player.health_timer = 10.;
                    did_lose_health = true; 
                }
            }
            if player.health_timer > 0. {
                player.health_timer = player.health_timer - time.delta_seconds();
            }
            // Now send this information back to the server 
            if did_lose_health { 
                if client_info.socket.is_none() {
                    return;
                }
        
                info!("Sending player info");
                let socket = client_info.socket.as_mut().unwrap();
                socket.set_nonblocking(true).expect("could not set non-blocking");
     
                    // println!("Old client position is {:?}", old_position);
                
                for client in client_list.0.iter()
                {
                    if player_num.0 == client.username[4..client.username.len()].parse::<u32>().unwrap()
                    {
                        let new_info = Some(PlayerInfo{
                            position: client.player_info.as_ref().unwrap().position.clone(),
                            velocity: client.player_info.as_ref().unwrap().velocity.clone(),
                            health: player.health,
                        });
                        let identifier = "PHIFC";
                        let message = serde_json::to_string(&new_info).expect("Failed to serialize");
                        let big_message = identifier.to_string() + &message;
                        match socket.send(big_message.as_bytes()) {
                            Ok(_) => {},
                            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                // Ignore WouldBlock errors
                            },
                            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                                println!("Socket is already connected");
                            },
                            Err(e) => {
                                println!("failed to send server updated position");
                                println!("Failed to send data: {:?}", e);
                            }
                        }
        
                    }
                }
                    
            }
        }
    }
}

pub fn ball_player_collisions(
    mut query: Query<(&mut Transform, &mut BallVelocity, &mut Ball), (With<Ball>, Without<Player>)>,
    mut player_query: Query<(&mut Transform, &mut Player), (With<Player>, Without<Ball>)>,
    mut healthbar_query: Query<
        (&mut Visibility, &mut Health),
        (With<Health>, Without<Ball>, Without<Player>),
    >,
    input_mouse: Res<Input<MouseButton>>,
    time: Res<Time>,
) {
    for (mut transform, mut ball_velocity, mut ball) in query.iter_mut() {
        let ball_radius = ball.radius * 3.;

        for (mut player_transform, mut player) in player_query.iter_mut() {
            let player_collision = bevy::sprite::collide_aabb::collide(
                player_transform.translation,
                Vec2::splat(PLAYER_SIZE),
                transform.translation,
                Vec2::new(ball_radius * 2., ball_radius * 2.),
            );

            if player_collision == Some(bevy::sprite::collide_aabb::Collision::Right)
                && player.health > 0
            {
                ball_velocity.velocity.x = -ball_velocity.velocity.x * ball.elasticity;
                ball_velocity.velocity.y = ball_velocity.velocity.y * ball.elasticity;
                transform.translation.x = transform.translation.x - 10.;
                if !input_mouse.pressed(MouseButton::Left) && player.health_timer <= 0. {
                    for (mut health_visibility, mut healthbar) in healthbar_query.iter_mut() {
                        if healthbar.lives == player.health
                            && healthbar.player_type == "player".to_string()
                        {
                            *health_visibility = Visibility::Hidden;
                        }
                    }
                    player.health = player.health - 1;
                    player.health_timer = 10.;
                }
            } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Left)
                && player.health > 0
            {
                ball_velocity.velocity.x = -ball_velocity.velocity.x * ball.elasticity;
                ball_velocity.velocity.y = ball_velocity.velocity.y * ball.elasticity;
                transform.translation.x = transform.translation.x + 10.;
                if !input_mouse.pressed(MouseButton::Left) && player.health_timer <= 0. {
                    for (mut health_visibility, mut healthbar) in healthbar_query.iter_mut() {
                        if healthbar.lives == player.health
                            && healthbar.player_type == "player".to_string()
                        {
                            *health_visibility = Visibility::Hidden;
                        }
                    }
                    player.health = player.health - 1;
                    player.health_timer = 10.;
                }
            } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Top)
                && player.health > 0
            {
                ball_velocity.velocity.y = -ball_velocity.velocity.y * ball.elasticity;
                ball_velocity.velocity.x = ball_velocity.velocity.x * ball.elasticity;
                transform.translation.y = transform.translation.y - 10.;
                if !input_mouse.pressed(MouseButton::Left) && player.health_timer <= 0. {
                    for (mut health_visibility, mut healthbar) in healthbar_query.iter_mut() {
                        if healthbar.lives == player.health
                            && healthbar.player_type == "player".to_string()
                        {
                            *health_visibility = Visibility::Hidden;
                        }
                    }
                    player.health = player.health - 1;
                    player.health_timer = 10.;
                }
            } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Bottom)
                && player.health > 0
            {
                ball_velocity.velocity.y = -ball_velocity.velocity.y * ball.elasticity;
                ball_velocity.velocity.x = ball_velocity.velocity.x * ball.elasticity;
                transform.translation.y = transform.translation.y + 10.;
                if !input_mouse.pressed(MouseButton::Left) && player.health_timer <= 0. {
                    for (mut health_visibility, mut healthbar) in healthbar_query.iter_mut() {
                        if healthbar.lives == player.health
                            && healthbar.player_type == "player".to_string()
                        {
                            *health_visibility = Visibility::Hidden;
                        }
                    }
                    player.health = player.health - 1;
                    player.health_timer = 10.;
                }
            }
            if player.health_timer > 0. {
                player.health_timer = player.health_timer - time.delta_seconds();
            }
        }
    }
}

pub fn ball_npc_collisions(
    mut query: Query<(&mut Transform, &mut BallVelocity, &mut Ball), (With<Ball>, Without<Player>)>,
    mut npc_query: Query<(&mut Transform, &mut NPC), (With<NPC>, Without<Ball>)>,
    mut healthbar_query: Query<
        (&mut Visibility, &mut Health),
        (With<Health>, Without<Ball>, Without<Player>),
    >,
    input_mouse: Res<Input<MouseButton>>,
    time: Res<Time>,
) {
    for (mut transform, mut ball_velocity, mut ball) in query.iter_mut() {
        let ball_radius = ball.radius * 3.;

        for (mut npc_transform, mut npc) in npc_query.iter_mut() {
            let npc_collision = bevy::sprite::collide_aabb::collide(
                npc_transform.translation,
                Vec2::splat(15.),
                transform.translation,
                Vec2::new(ball_radius * 2., ball_radius * 2.),
            );

            if npc_collision == Some(bevy::sprite::collide_aabb::Collision::Right) && npc.health > 0
            {
                ball_velocity.velocity.x = -ball_velocity.velocity.x * ball.elasticity;
                ball_velocity.velocity.y = ball_velocity.velocity.y * ball.elasticity;
                transform.translation.x = transform.translation.x - 10.;
                if !input_mouse.pressed(MouseButton::Left) && npc.health_timer <= 0. {
                    for (mut health_visibility, mut healthbar) in healthbar_query.iter_mut() {
                        if healthbar.lives == npc.health
                            && healthbar.player_type == "npc".to_string()
                        {
                            *health_visibility = Visibility::Hidden;
                        }
                    }
                    npc.health = npc.health - 1;
                    npc.health_timer = 10.;
                }
            } else if npc_collision == Some(bevy::sprite::collide_aabb::Collision::Left)
                && npc.health > 0
            {
                ball_velocity.velocity.x = -ball_velocity.velocity.x * ball.elasticity;
                ball_velocity.velocity.y = ball_velocity.velocity.y * ball.elasticity;
                transform.translation.x = transform.translation.x + 10.;
                if !input_mouse.pressed(MouseButton::Left) && npc.health_timer <= 0. {
                    for (mut health_visibility, mut healthbar) in healthbar_query.iter_mut() {
                        if healthbar.lives == npc.health
                            && healthbar.player_type == "npc".to_string()
                        {
                            *health_visibility = Visibility::Hidden;
                        }
                    }
                    npc.health = npc.health - 1;
                    npc.health_timer = 10.;
                }
            } else if npc_collision == Some(bevy::sprite::collide_aabb::Collision::Top)
                && npc.health > 0
            {
                ball_velocity.velocity.y = -ball_velocity.velocity.y * ball.elasticity;
                ball_velocity.velocity.x = ball_velocity.velocity.x * ball.elasticity;
                transform.translation.y = transform.translation.y - 10.;
                if !input_mouse.pressed(MouseButton::Left) && npc.health_timer <= 0. {
                    for (mut health_visibility, mut healthbar) in healthbar_query.iter_mut() {
                        if healthbar.lives == npc.health
                            && healthbar.player_type == "npc".to_string()
                        {
                            *health_visibility = Visibility::Hidden;
                        }
                    }
                    npc.health = npc.health - 1;
                    npc.health_timer = 10.;
                }
            } else if npc_collision == Some(bevy::sprite::collide_aabb::Collision::Bottom)
                && npc.health > 0
            {
                ball_velocity.velocity.y = -ball_velocity.velocity.y * ball.elasticity;
                ball_velocity.velocity.x = ball_velocity.velocity.x * ball.elasticity;
                transform.translation.y = transform.translation.y + 10.;
                if !input_mouse.pressed(MouseButton::Left) && npc.health_timer <= 0. {
                    for (mut health_visibility, mut healthbar) in healthbar_query.iter_mut() {
                        if healthbar.lives == npc.health
                            && healthbar.player_type == "npc".to_string()
                        {
                            *health_visibility = Visibility::Hidden;
                        }
                    }
                    npc.health = npc.health - 1;
                    npc.health_timer = 10.;
                }
            }
            if npc.health_timer > 0. {
                npc.health_timer = npc.health_timer - time.delta_seconds();
            }
        }
    }
}

pub fn bounce_m2(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut BallVelocity, &mut Ball), (With<Ball>, Without<Player>)>,
    //health hitbox
    mut player_query: Query<(&mut Transform, &mut Player), (With<Player>, Without<Ball>)>,
    mut healthbar_query: Query<
        (&mut Visibility, &mut Health),
        (With<Health>, Without<Ball>, Without<Player>),
    >,
    input_mouse: Res<Input<MouseButton>>,
) {
    for (mut transform, mut ball_velocity, mut ball) in query.iter_mut() {
        //ball radius on screen
        let ball_radius = ball.radius * 3.;

        // Find the new translation for the x and y for the ball
        let mut new_translation_x = (transform.translation.x
            + (ball_velocity.velocity.x * time.delta_seconds()))
        .clamp(-(1280. / 2.) + ball_radius, 1280. / 2. - ball_radius);

        let mut new_translation_y = (transform.translation.y
            + (ball_velocity.velocity.y * time.delta_seconds()))
        .clamp(-(720. / 2.) + ball_radius, 720. / 2. - ball_radius);

        let new_translation = Vec3::new(
            new_translation_x,
            new_translation_y,
            transform.translation.z,
        );
        if unsafe { MAP == 4 } {
            let coral_size = Vec2::new(150., 150.);
            let coral1 = bevy::sprite::collide_aabb::collide(
                Vec3::new(0., 180., 2.),
                coral_size,
                new_translation,
                Vec2::new(ball_radius * 2., ball_radius * 2.),
            );

            let coral2 = bevy::sprite::collide_aabb::collide(
                Vec3::new(0., -180., 2.),
                coral_size,
                new_translation,
                Vec2::new(ball_radius * 2., ball_radius * 2.),
            );

            let coral3 = bevy::sprite::collide_aabb::collide(
                Vec3::new(-320., 180., 2.),
                coral_size,
                new_translation,
                Vec2::new(ball_radius * 2., ball_radius * 2.),
            );

            let coral4 = bevy::sprite::collide_aabb::collide(
                Vec3::new(-320., -180., 2.),
                coral_size,
                new_translation,
                Vec2::new(ball_radius * 2., ball_radius * 2.),
            );

            let coral5 = bevy::sprite::collide_aabb::collide(
                Vec3::new(320., 180., 2.),
                coral_size,
                new_translation,
                Vec2::new(ball_radius * 2., ball_radius * 2.),
            );

            let coral6 = bevy::sprite::collide_aabb::collide(
                Vec3::new(320., -180., 2.),
                coral_size,
                new_translation,
                Vec2::new(ball_radius * 2., ball_radius * 2.),
            );

            if coral1 == Some(bevy::sprite::collide_aabb::Collision::Left) {
                ball_velocity.velocity.x = -ball_velocity.velocity.x * 0.8 * ball.elasticity;
                ball_velocity.velocity.y = ball_velocity.velocity.y * 0.8 * ball.elasticity;
                new_translation_x = Vec3::new(0., 180., 2.).x + coral_size.x / 2. + ball_radius;
            } else if coral1 == Some(bevy::sprite::collide_aabb::Collision::Right) {
                ball_velocity.velocity.x = -ball_velocity.velocity.x * 0.8 * ball.elasticity;
                ball_velocity.velocity.y = ball_velocity.velocity.y * 0.8 * ball.elasticity;
                new_translation_x = Vec3::new(0., 180., 2.).x - coral_size.x / 2. - ball_radius;
            } else if coral1 == Some(bevy::sprite::collide_aabb::Collision::Top) {
                ball_velocity.velocity.y = -ball_velocity.velocity.y * 0.8 * ball.elasticity;
                ball_velocity.velocity.x = ball_velocity.velocity.x * 0.8 * ball.elasticity;
                new_translation_y = Vec3::new(0., 180., 2.).y - coral_size.y / 2. - ball_radius;
            } else if coral1 == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
                ball_velocity.velocity.y = -ball_velocity.velocity.y * 0.8 * ball.elasticity;
                ball_velocity.velocity.x = ball_velocity.velocity.x * 0.8 * ball.elasticity;
                new_translation_y = Vec3::new(0., 180., 2.).y + coral_size.y / 2. + ball_radius;
            }

            if coral2 == Some(bevy::sprite::collide_aabb::Collision::Left) {
                ball_velocity.velocity.x = -ball_velocity.velocity.x * 0.8 * ball.elasticity;
                ball_velocity.velocity.y = ball_velocity.velocity.y * 0.8 * ball.elasticity;
                new_translation_x = Vec3::new(0., -180., 2.).x + coral_size.x / 2. + ball_radius;
            } else if coral2 == Some(bevy::sprite::collide_aabb::Collision::Right) {
                ball_velocity.velocity.x = -ball_velocity.velocity.x * 0.8 * ball.elasticity;
                ball_velocity.velocity.y = ball_velocity.velocity.y * 0.8 * ball.elasticity;
                new_translation_x = Vec3::new(0., -180., 2.).x - coral_size.x / 2. - ball_radius;
            } else if coral2 == Some(bevy::sprite::collide_aabb::Collision::Top) {
                ball_velocity.velocity.y = -ball_velocity.velocity.y * 0.8 * ball.elasticity;
                ball_velocity.velocity.x = ball_velocity.velocity.x * 0.8 * ball.elasticity;
                new_translation_y = Vec3::new(0., -180., 2.).y - coral_size.y / 2. - ball_radius;
            } else if coral2 == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
                ball_velocity.velocity.y = -ball_velocity.velocity.y * 0.8 * ball.elasticity;
                ball_velocity.velocity.x = ball_velocity.velocity.x * 0.8 * ball.elasticity;
                new_translation_y = Vec3::new(0., -180., 2.).y + coral_size.y / 2. + ball_radius;
            }

            if coral3 == Some(bevy::sprite::collide_aabb::Collision::Left) {
                ball_velocity.velocity.x = -ball_velocity.velocity.x * 0.8 * ball.elasticity;
                ball_velocity.velocity.y = ball_velocity.velocity.y * 0.8 * ball.elasticity;
                new_translation_x = Vec3::new(-320., 180., 2.).x + coral_size.x / 2. + ball_radius;
            } else if coral3 == Some(bevy::sprite::collide_aabb::Collision::Right) {
                ball_velocity.velocity.x = -ball_velocity.velocity.x * 0.8 * ball.elasticity;
                ball_velocity.velocity.y = ball_velocity.velocity.y * 0.8 * ball.elasticity;
                new_translation_x = Vec3::new(-320., 180., 2.).x - coral_size.x / 2. - ball_radius;
            } else if coral3 == Some(bevy::sprite::collide_aabb::Collision::Top) {
                ball_velocity.velocity.y = -ball_velocity.velocity.y * 0.8 * ball.elasticity;
                ball_velocity.velocity.x = ball_velocity.velocity.x * 0.8 * ball.elasticity;
                new_translation_y = Vec3::new(-320., 180., 2.).y - coral_size.y / 2. - ball_radius;
            } else if coral3 == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
                ball_velocity.velocity.y = -ball_velocity.velocity.y * 0.8 * ball.elasticity;
                ball_velocity.velocity.x = ball_velocity.velocity.x * 0.8 * ball.elasticity;
                new_translation_y = Vec3::new(-320., 180., 2.).y + coral_size.y / 2. + ball_radius;
            }

            if coral4 == Some(bevy::sprite::collide_aabb::Collision::Left) {
                ball_velocity.velocity.x = -ball_velocity.velocity.x * 0.8 * ball.elasticity;
                ball_velocity.velocity.y = ball_velocity.velocity.y * 0.8 * ball.elasticity;
                new_translation_x = Vec3::new(-320., -180., 2.).x + coral_size.x / 2. + ball_radius;
            } else if coral4 == Some(bevy::sprite::collide_aabb::Collision::Right) {
                ball_velocity.velocity.x = -ball_velocity.velocity.x * 0.8 * ball.elasticity;
                ball_velocity.velocity.y = ball_velocity.velocity.y * 0.8 * ball.elasticity;
                new_translation_x = Vec3::new(-320., -180., 2.).x - coral_size.x / 2. - ball_radius;
            } else if coral4 == Some(bevy::sprite::collide_aabb::Collision::Top) {
                ball_velocity.velocity.y = -ball_velocity.velocity.y * 0.8 * ball.elasticity;
                ball_velocity.velocity.x = ball_velocity.velocity.x * 0.8 * ball.elasticity;
                new_translation_y = Vec3::new(-320., -180., 2.).y - coral_size.y / 2. - ball_radius;
            } else if coral4 == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
                ball_velocity.velocity.y = -ball_velocity.velocity.y * 0.8 * ball.elasticity;
                ball_velocity.velocity.x = ball_velocity.velocity.x * 0.8 * ball.elasticity;
                new_translation_y = Vec3::new(-320., -180., 2.).y + coral_size.y / 2. + ball_radius;
            }

            if coral5 == Some(bevy::sprite::collide_aabb::Collision::Left) {
                ball_velocity.velocity.x = -ball_velocity.velocity.x * 0.8 * ball.elasticity;
                ball_velocity.velocity.y = ball_velocity.velocity.y * 0.8 * ball.elasticity;
                new_translation_x = Vec3::new(320., 180., 2.).x + coral_size.x / 2. + ball_radius;
            } else if coral5 == Some(bevy::sprite::collide_aabb::Collision::Right) {
                ball_velocity.velocity.x = -ball_velocity.velocity.x * 0.8 * ball.elasticity;
                ball_velocity.velocity.y = ball_velocity.velocity.y * 0.8 * ball.elasticity;
                new_translation_x = Vec3::new(320., 180., 2.).x - coral_size.x / 2. - ball_radius;
            } else if coral5 == Some(bevy::sprite::collide_aabb::Collision::Top) {
                ball_velocity.velocity.y = -ball_velocity.velocity.y * 0.8 * ball.elasticity;
                ball_velocity.velocity.x = ball_velocity.velocity.x * 0.8 * ball.elasticity;
                new_translation_y = Vec3::new(320., 180., 2.).y - coral_size.y / 2. - ball_radius;
            } else if coral5 == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
                ball_velocity.velocity.y = -ball_velocity.velocity.y * 0.8 * ball.elasticity;
                ball_velocity.velocity.x = ball_velocity.velocity.x * 0.8 * ball.elasticity;
                new_translation_y = Vec3::new(320., 180., 2.).y + coral_size.y / 2. + ball_radius;
            }

            if coral6 == Some(bevy::sprite::collide_aabb::Collision::Left) {
                ball_velocity.velocity.x = -ball_velocity.velocity.x * 0.8 * ball.elasticity;
                ball_velocity.velocity.y = ball_velocity.velocity.y * 0.8 * ball.elasticity;
                new_translation_x = Vec3::new(320., -180., 2.).x + coral_size.x / 2. + ball_radius;
            } else if coral6 == Some(bevy::sprite::collide_aabb::Collision::Right) {
                ball_velocity.velocity.x = -ball_velocity.velocity.x * 0.8 * ball.elasticity;
                ball_velocity.velocity.y = ball_velocity.velocity.y * 0.8 * ball.elasticity;
                new_translation_x = Vec3::new(320., -180., 2.).x - coral_size.x / 2. - ball_radius;
            } else if coral6 == Some(bevy::sprite::collide_aabb::Collision::Top) {
                ball_velocity.velocity.y = -ball_velocity.velocity.y * 0.8 * ball.elasticity;
                ball_velocity.velocity.x = ball_velocity.velocity.x * 0.8 * ball.elasticity;
                new_translation_y = Vec3::new(320., -180., 2.).y - coral_size.y / 2. - ball_radius;
            } else if coral6 == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
                ball_velocity.velocity.y = -ball_velocity.velocity.y * 0.8 * ball.elasticity;
                ball_velocity.velocity.x = ball_velocity.velocity.x * 0.8 * ball.elasticity;
                new_translation_y = Vec3::new(320., -180., 2.).y + coral_size.y / 2. + ball_radius;
            }
        }

        ball.prev_pos = transform.translation;

        // Move ball
        transform.translation.x = new_translation_x;
        transform.translation.y = new_translation_y;

        // Bounce when hitting the screen edges
        if transform.translation.x.abs() == WIN_W / 2.0 - ball_radius {
            ball_velocity.velocity.x = -ball_velocity.velocity.x * ball.elasticity;
            ball_velocity.velocity.y = ball_velocity.velocity.y * ball.elasticity;
        }
        if transform.translation.y.abs() == WIN_H / 2.0 - ball_radius {
            ball_velocity.velocity.y = -ball_velocity.velocity.y * ball.elasticity;
            ball_velocity.velocity.x = ball_velocity.velocity.x * ball.elasticity;
        }
    }
}

pub fn bounce_balls(
    mut query: Query<(&mut Transform, &mut BallVelocity, &mut Ball), (With<Ball>, Without<Player>)>,
) {
    // for debugging
    let mut combinations = query.iter_combinations_mut();
    while let Some([mut ball1query, mut ball2query]) = combinations.fetch_next() {
        let (mut ball1_transform, mut ball1_velocity, mut ball1) = ball1query;
        let (mut ball2_transform, mut ball2_velocity, mut ball2) = ball2query;

        let ball1_radius = ball1.radius * 3.;
        let ball2_radius = ball2.radius * 3.;

        let ball_collision = bevy::sprite::collide_aabb::collide(
            ball2_transform.translation,
            Vec2::new(ball2_radius * 2., ball2_radius * 2.),
            ball1_transform.translation,
            Vec2::new(ball1_radius * 2., ball1_radius * 2.),
        );

        let prev_collision = bevy::sprite::collide_aabb::collide(
            ball2.prev_pos,
            Vec2::new(ball2_radius * 2., ball2_radius * 2.),
            ball1.prev_pos,
            Vec2::new(ball1_radius * 2., ball1_radius * 2.),
        );

        let mut new_velocity;
        let mut new_velocity_2;

        if ball_collision == Some(bevy::sprite::collide_aabb::Collision::Left)
            || ball_collision == Some(bevy::sprite::collide_aabb::Collision::Right)
            || ball_collision == Some(bevy::sprite::collide_aabb::Collision::Top)
            || ball_collision == Some(bevy::sprite::collide_aabb::Collision::Bottom)
                && prev_collision == None
        {
            //Find time t where the 2 balls collided
            // Using equations: d = sqrt((ball1.x - ball2.x)^2 + (ball1.y - ball2.y)^2), y' = velocity.y * t + y, x' = velocity.x * t + x, and quadratic formula
            let a = ball1_velocity.velocity.x * ball1_velocity.velocity.x
                + ball1_velocity.velocity.y * ball1_velocity.velocity.y
                + ball2_velocity.velocity.x * ball2_velocity.velocity.x
                + ball2_velocity.velocity.y * ball2_velocity.velocity.y
                - 2. * ball1_velocity.velocity.x * ball2_velocity.velocity.x
                - 2. * ball1_velocity.velocity.y * ball2_velocity.velocity.y;
            let b = 2. * ball1_velocity.velocity.x * ball1_transform.translation.x
                + 2. * ball1_velocity.velocity.y * ball1_transform.translation.y
                + 2. * ball2_velocity.velocity.x * ball2_transform.translation.x
                + 2. * ball2_velocity.velocity.y * ball2_transform.translation.y
                - 2. * ball1_velocity.velocity.x * ball2_transform.translation.x
                - 2. * ball2_velocity.velocity.x * ball1_transform.translation.x
                - 2. * ball2_velocity.velocity.y * ball1_transform.translation.y
                - 2. * ball2_transform.translation.y * ball1_velocity.velocity.y;
            let c = ball1_transform.translation.x * ball1_transform.translation.x
                + ball2_transform.translation.x * ball2_transform.translation.x
                + ball1_transform.translation.y * ball1_transform.translation.y
                + ball2_transform.translation.y * ball2_transform.translation.y
                - 2. * ball1_transform.translation.x * ball2_transform.translation.x
                - 2. * ball1_transform.translation.y * ball2_transform.translation.y
                - (ball1_radius + ball2_radius) * (ball1_radius + ball2_radius);
            let d = b * b - 4. * a * c;
            //let changet = (-b + (b * b - 4. * a * c).sqrt()) / (2. * a);
            let negchange = (-b - (b * b - 4. * a * c).sqrt()) / (2. * a);

            if !negchange.is_nan() && !negchange.is_infinite() && b < (-0.000001) && d > 0. {
                ball1_transform.translation.x =
                    ball1_velocity.velocity.x * negchange + ball1_transform.translation.x;
                ball1_transform.translation.y =
                    ball1_velocity.velocity.y * negchange + ball1_transform.translation.y;

                ball2_transform.translation.x =
                    ball2_velocity.velocity.x * negchange + ball2_transform.translation.x;
                ball2_transform.translation.y =
                    ball2_velocity.velocity.y * negchange + ball2_transform.translation.y;

                let ball1_mass = (4. / 3.) * PI * (ball1_radius).powf(3.) * ball1.density;
                let ball2_mass = (4. / 3.) * PI * (ball2_radius).powf(3.) * ball2.density;

                new_velocity = ((ball1_mass - ball2_mass) / (ball2_mass + ball1_mass))
                    * ball1_velocity.velocity
                    + ((2. * ball2_mass) / (ball2_mass + ball1_mass)) * ball2_velocity.velocity;
                new_velocity_2 = ((2. * ball1_mass) / (ball2_mass + ball1_mass))
                    * ball1_velocity.velocity
                    + ((ball2_mass - ball1_mass) / (ball2_mass + ball1_mass))
                        * ball1_velocity.velocity;

                new_velocity = new_velocity * ball1.elasticity * ball2.elasticity;
                new_velocity_2 = new_velocity_2 * ball1.elasticity * ball2.elasticity;

                ball1_velocity.velocity = new_velocity;
                ball2_velocity.velocity = new_velocity_2;
            }
        }
    }
}

fn bat_hitbox(
    mut hitbox: Query<&mut Sprite, (With<Hitbox>, Without<Bat>)>,
    input_mouse: Res<Input<MouseButton>>,
) {
    let mut color_hitbox = hitbox.single_mut();

    if input_mouse.pressed(MouseButton::Left) {
        // Left button was pressed
        color_hitbox.color = Color::rgba(240., 140., 100., 0.2);
    } else if !input_mouse.pressed(MouseButton::Left) {
        color_hitbox.color = Color::rgba(240., 140., 100., 0.);
    }
}

fn bat_hitbox_mult(
    mut hitbox: Query<&mut Sprite, (With<Hitbox>, With<Player>, Without<Bat>)>,
    input_mouse: Res<Input<MouseButton>>,
) {
    let mut color_hitbox = hitbox.single_mut();

    if input_mouse.pressed(MouseButton::Left) {
        // Left button was pressed
        color_hitbox.color = Color::rgba(240., 140., 100., 0.2);
    } else {
        color_hitbox.color = Color::rgba(240., 140., 100., 0.);
    }
}

fn friction_map1(
    mut query: Query<(&Transform, &mut BallVelocity), With<Ball>>,
    rug: Query<(&Transform, &Rug), With<Rug>>,
    time: Res<Time>,
) {
    let (rug_transform, rug) = rug.single();
    let rug_size = Vec2::new(720., 500.);
    let deltat = time.delta_seconds();

    for (ball_transform, mut ball_velocity) in query.iter_mut() {
        // If the ball is on the rug, slow it down using the rugs coefficient of friction
        let rug_collision = bevy::sprite::collide_aabb::collide(
            rug_transform.translation,
            rug_size,
            ball_transform.translation,
            Vec2::new(BALL_SIZE, BALL_SIZE),
        );
        if (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Right))
            || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Left))
            || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Top))
            || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Bottom))
            || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Inside))
        {
            let newvx;
            let newvy;

            //Caclulate the new ball velocity using v' = v - G * coefficient of friction * deltat
            if ball_velocity.velocity.x < 0. {
                newvx = ball_velocity.velocity.x + G * rug.friction * deltat;
                if newvx < (-1. * MIN_BALL_VELOCITY) {
                    ball_velocity.velocity.x = newvx;
                }
            } else {
                newvx = ball_velocity.velocity.x - G * rug.friction * deltat;
                if newvx > MIN_BALL_VELOCITY {
                    ball_velocity.velocity.x = newvx;
                }
            }

            if ball_velocity.velocity.y < 0. {
                newvy = ball_velocity.velocity.y + G * rug.friction * deltat;
                if newvx < (-1. * MIN_BALL_VELOCITY) {
                    ball_velocity.velocity.y = newvy;
                }
            } else {
                newvy = ball_velocity.velocity.y - G * rug.friction * deltat;
                if newvy > MIN_BALL_VELOCITY {
                    ball_velocity.velocity.y = newvy;
                }
            }

        // If the ball is not on the rug, slow it down using the floors coefficient of friction (BASE_FRICTION)
        } else {
            let newvx;
            let newvy;

            //Caclulate the new ball velocity using v' = v - G * coefficient of friction * deltat
            if ball_velocity.velocity.x < 0. {
                newvx = ball_velocity.velocity.x + G * BASE_FRICTION * deltat;
                if newvx < (-1. * MIN_BALL_VELOCITY) {
                    ball_velocity.velocity.x = newvx;
                }
            } else {
                newvx = ball_velocity.velocity.x - G * BASE_FRICTION * deltat;
                if newvx > MIN_BALL_VELOCITY {
                    ball_velocity.velocity.x = newvx;
                }
            }

            if ball_velocity.velocity.y < 0. {
                newvy = ball_velocity.velocity.y + G * BASE_FRICTION * deltat;
                if newvx < (-1. * MIN_BALL_VELOCITY) {
                    ball_velocity.velocity.y = newvy;
                }
            } else {
                newvy = ball_velocity.velocity.y - G * BASE_FRICTION * deltat;
                if newvy > MIN_BALL_VELOCITY {
                    ball_velocity.velocity.y = newvy;
                }
            }
        }
    }
}

fn friction_map3(mut query: Query<(&Transform, &mut BallVelocity), With<Ball>>, time: Res<Time>) {
    let deltat = time.delta_seconds();

    for (ball_transform, mut ball_velocity) in query.iter_mut() {
        let newvx;
        let newvy;

        //Caclulate the new ball velocity using v' = v - G * coefficient of friction * deltat
        if ball_velocity.velocity.x < 0. {
            newvx = ball_velocity.velocity.x + G * 0.05 * deltat;
            if newvx < (-1. * MIN_BALL_VELOCITY) {
                ball_velocity.velocity.x = newvx;
            }
        } else {
            newvx = ball_velocity.velocity.x - G * 0.05 * deltat;
            if newvx > MIN_BALL_VELOCITY {
                ball_velocity.velocity.x = newvx;
            }
        }

        if ball_velocity.velocity.y < 0. {
            newvy = ball_velocity.velocity.y + G * 0.05 * deltat;
            if newvx < (-1. * MIN_BALL_VELOCITY) {
                ball_velocity.velocity.y = newvy;
            }
        } else {
            newvy = ball_velocity.velocity.y - G * 0.05 * deltat;
            if newvy > MIN_BALL_VELOCITY {
                ball_velocity.velocity.y = newvy;
            }
        }
    }
}

fn friction_map4(mut query: Query<(&Transform, &mut BallVelocity), With<Ball>>, time: Res<Time>) {
    let deltat = time.delta_seconds();

    for (ball_transform, mut ball_velocity) in query.iter_mut() {
        let newvx;
        let newvy;

        //Caclulate the new ball velocity using v' = v - G * coefficient of friction * deltat
        if ball_velocity.velocity.x < 0. {
            newvx = ball_velocity.velocity.x + G * 0.75 * deltat;
            if newvx < (-1. * MIN_BALL_VELOCITY) {
                ball_velocity.velocity.x = newvx;
            }
        } else {
            newvx = ball_velocity.velocity.x - G * 0.75 * deltat;
            if newvx > MIN_BALL_VELOCITY {
                ball_velocity.velocity.x = newvx;
            }
        }

        if ball_velocity.velocity.y < 0. {
            newvy = ball_velocity.velocity.y + G * 0.75 * deltat;
            if newvx < (-1. * MIN_BALL_VELOCITY) {
                ball_velocity.velocity.y = newvy;
            }
        } else {
            newvy = ball_velocity.velocity.y - G * 0.75 * deltat;
            if newvy > MIN_BALL_VELOCITY {
                ball_velocity.velocity.y = newvy;
            }
        }
    }
}

//bat swing function, now on RELEASE of mouse button (based on cursor)

fn swing(
    //mut commands: Commands,
    input_mouse: Res<Input<MouseButton>>,
    input: Res<Input<KeyCode>>,
    mut query: Query<
        (&mut Ball, &mut BallVelocity, &mut Transform),
        (With<Ball>, Without<Hitbox>, Without<Bat>, Without<Player>),
    >,
    mut query_bat: Query<
        &mut Transform,
        (With<Bat>, Without<Hitbox>, Without<Ball>, Without<Player>),
    >,
    //cursor_events: ResMut<Events<CursorMoved>>,
    mut hitbox: Query<
        (&mut Transform, &mut Hitbox),
        (With<Hitbox>, Without<Ball>, Without<Ball>, Without<Player>),
    >,
    window: Query<&Window>,
    player: Query<&Transform, (With<Player>, Without<Hitbox>, Without<Bat>, Without<Ball>)>,
) {
    let (mut hitbox_transform, hitbox) = hitbox.single_mut();

    static mut MOUSE_BUTTON_PRESSED: bool = false;
    static mut BAT_TRANSFORMED: bool = false;
    static mut MOUSE_BUTTON_JUST_RELEASED: bool = false;
    //let mut mouse_position: Vec2;
    let mut bat_transform = query_bat.single_mut();
    let player_transform = player.single();

    if input_mouse.just_pressed(MouseButton::Left) {
        // Mouse button was just pressed
        unsafe {
            MOUSE_BUTTON_PRESSED = true;
            BAT_TRANSFORMED = false;
            MOUSE_BUTTON_JUST_RELEASED = false;
        }
        //println!("Mouse button pressed");
    } else if input_mouse.just_released(MouseButton::Left) {
        // Mouse button was just released
        unsafe {
            if MOUSE_BUTTON_PRESSED {
                MOUSE_BUTTON_PRESSED = false;
                BAT_TRANSFORMED = true;
                MOUSE_BUTTON_JUST_RELEASED = true;
                //println!("Mouse button released");
            }
        }
    }

    /*let mut cursor_event_reader = cursor_events.get_reader();
    for event in cursor_event_reader.iter(&cursor_events) {
        // Update the mouse position
        mouse_position = event.position;
        //println!("Mouse position changed");
    }*/

    //for (bat, mut bat_transform) in query_bat.iter_mut() {
    if unsafe { MOUSE_BUTTON_PRESSED } {
        // Left mouse button is pressed, set the bat to horizontal
        bat_transform.scale.y = -bat_transform.scale.y.abs();
    } else if unsafe { BAT_TRANSFORMED } {
        bat_transform.scale.y = bat_transform.scale.y.abs();
    }
    //}

    if let Some(mouse_position) = window.single().physical_cursor_position() {
        //println!("Cursor is inside window {:?}", mouse_position);
        // Move bat to the same side of the player as the mouse
        if ((mouse_position.x - WIN_W) / 2.) > player_transform.translation.x {
            bat_transform.translation = player_transform.translation;
            bat_transform.translation.x = bat_transform.translation.x + 8.;
            bat_transform.scale.x = -bat_transform.scale.x.abs();

            hitbox_transform.translation = bat_transform.translation;
            hitbox_transform.translation.x = hitbox_transform.translation.x + hitbox.size.x / 2.;
            hitbox_transform.translation.y = hitbox_transform.translation.y - 5.;
        } else {
            bat_transform.translation = player_transform.translation;
            bat_transform.translation.x = bat_transform.translation.x - 5.;
            bat_transform.scale.x = bat_transform.scale.x.abs();

            hitbox_transform.translation = bat_transform.translation;
            hitbox_transform.translation.x = hitbox_transform.translation.x - hitbox.size.x / 2.;
            hitbox_transform.translation.y = hitbox_transform.translation.y - 5.;
        }
        if unsafe { MOUSE_BUTTON_JUST_RELEASED } {
            for (mut ball, mut ball_velocity, mut ball_transform) in query.iter_mut() {
                let bat_to_ball_collision = bevy::sprite::collide_aabb::collide(
                    hitbox_transform.translation,
                    hitbox.size,
                    ball_transform.translation,
                    Vec2::new(BALL_SIZE, BALL_SIZE),
                );

                if (bat_to_ball_collision == Some(bevy::sprite::collide_aabb::Collision::Right))
                    || (bat_to_ball_collision == Some(bevy::sprite::collide_aabb::Collision::Left))
                    || (bat_to_ball_collision == Some(bevy::sprite::collide_aabb::Collision::Top))
                    || (bat_to_ball_collision
                        == Some(bevy::sprite::collide_aabb::Collision::Bottom))
                    || (bat_to_ball_collision
                        == Some(bevy::sprite::collide_aabb::Collision::Inside))
                {
                    ball_velocity.velocity = Vec3::splat(0.);
                    let change_x =
                        (((mouse_position.x - WIN_W) / 2.) - ball_transform.translation.x).abs();
                    let change_y =
                        ((-(mouse_position.y - WIN_H) / 2.) - ball_transform.translation.y).abs();
                    let mut new_velocity = Vec3::new(change_x, change_y, 0.);
                    new_velocity = new_velocity.normalize_or_zero();

                    if ((mouse_position.x - WIN_W) / 2.) > ball_transform.translation.x {
                        new_velocity.x = new_velocity.x;
                    } else {
                        new_velocity.x = -1. * new_velocity.x;
                    }

                    if (-(mouse_position.y - WIN_H) / 2.) > ball_transform.translation.y {
                        new_velocity.y = new_velocity.y;
                    } else {
                        new_velocity.y = -1. * new_velocity.y;
                    }

                    new_velocity.x = new_velocity.x * 500.;
                    new_velocity.y = new_velocity.y * 500.;

                    // if Q is pressed, backspin -> ball moves slower
                    if input.pressed(KeyCode::Q) {
                        new_velocity *= 0.5;
                    }

                    // if E is pressed, topspin -> ball moves faster
                    if input.pressed(KeyCode::E) {
                        new_velocity *= 1.5;
                    }

                    ball_velocity.velocity = new_velocity * ball.elasticity;
                }

                // let ball_position = ball_velocity.velocity.truncate();
                // println!("Ball position: {:?}", ball_position);

                /*let direction =  MOUSE_POSITION - ball_velocity.velocity.truncate();;
                println!("Direction: {:?}", direction);


                // Normalize the direction and set the ball's velocity
                let normalized_direction = direction.normalize_or_zero();
                //println!("Normalized direction: {:?}", normalized_direction);

                ball_velocity.velocity = Vec3::new(
                    normalized_direction.x * HIT_POWER.x,
                    normalized_direction.y * HIT_POWER.y,
                    0.0,
                );
                println!("Ball velocity: {:?}", ball_velocity.velocity);*/
            }

            // Reset the flags for the next interaction
            unsafe {
                MOUSE_BUTTON_JUST_RELEASED = false;
                BAT_TRANSFORMED = false;
            }
        }
    }
}

fn update_balls(
    mut ball_list: EventReader<ClientBallInfo>,
    mut ball_query: Query<
    (&mut Transform, &mut BallVelocity, &BallNumber),
    With<Ball>>,
)
{
    for event in ball_list.iter()
    {
        for (mut transform, mut velocity, ball_num) in ball_query.iter_mut()
        {
            // println!("updating ball {}", ball_num.number);
            for ball in event.data.iter()
            {
                if ball.ball_number.number == ball_num.number
                {
                    transform.translation.x = ball.position.0;
                    transform.translation.y = ball.position.1;
                    *velocity = ball.velocity.clone();
                }
            }
        }
    }
}


fn swing_m4(
    //mut commands: Commands,
    input_mouse: Res<Input<MouseButton>>,
    input: Res<Input<KeyCode>>,
    mut query: Query<
        (&mut Ball, &mut BallVelocity, &mut Transform),
        (With<Ball>, Without<Hitbox>, Without<Bat>, Without<Player>),
    >,
    mut query_bat: Query<
        &mut Transform,
        (With<Bat>, Without<Hitbox>, Without<Ball>, Without<Player>),
    >,
    //cursor_events: ResMut<Events<CursorMoved>>,
    mut hitbox: Query<
        (&mut Transform, &mut Hitbox),
        (With<Hitbox>, Without<Ball>, Without<Ball>, Without<Player>),
    >,
    window: Query<&Window>,
    player: Query<&Transform, (With<Player>, Without<Hitbox>, Without<Bat>, Without<Ball>)>,
) {
    let (mut hitbox_transform, hitbox) = hitbox.single_mut();

    static mut MOUSE_BUTTON_PRESSED: bool = false;
    static mut BAT_TRANSFORMED: bool = false;
    static mut MOUSE_BUTTON_JUST_RELEASED: bool = false;
    //let mut mouse_position: Vec2;
    let mut bat_transform = query_bat.single_mut();
    let player_transform = player.single();

    if input_mouse.just_pressed(MouseButton::Left) {
        // Mouse button was just pressed
        unsafe {
            MOUSE_BUTTON_PRESSED = true;
            BAT_TRANSFORMED = false;
            MOUSE_BUTTON_JUST_RELEASED = false;
        }
        //println!("Mouse button pressed");
    } else if input_mouse.just_released(MouseButton::Left) {
        // Mouse button was just released
        unsafe {
            if MOUSE_BUTTON_PRESSED {
                MOUSE_BUTTON_PRESSED = false;
                BAT_TRANSFORMED = true;
                MOUSE_BUTTON_JUST_RELEASED = true;
                //println!("Mouse button released");
            }
        }
    }

    /*let mut cursor_event_reader = cursor_events.get_reader();
    for event in cursor_event_reader.iter(&cursor_events) {
        // Update the mouse position
        mouse_position = event.position;
        //println!("Mouse position changed");
    }*/

    //for (bat, mut bat_transform) in query_bat.iter_mut() {
    if unsafe { MOUSE_BUTTON_PRESSED } {
        // Left mouse button is pressed, set the bat to horizontal
        bat_transform.scale.y = -0.175;
    } else if unsafe { BAT_TRANSFORMED } {
        bat_transform.scale.y = 0.175;
    }
    //}

    if let Some(mouse_position) = window.single().physical_cursor_position() {
        //println!("Cursor is inside window {:?}", mouse_position);
        // Move bat to the same side of the player as the mouse
        if ((mouse_position.x - WIN_W) / 2.) > player_transform.translation.x {
            bat_transform.translation = player_transform.translation;
            bat_transform.translation.x = bat_transform.translation.x + 8.;
            bat_transform.scale.x = -0.175;

            hitbox_transform.translation = bat_transform.translation;
            hitbox_transform.translation.x = hitbox_transform.translation.x + 20.;
            hitbox_transform.translation.y = hitbox_transform.translation.y - 5.;
        } else {
            bat_transform.translation = player_transform.translation;
            bat_transform.translation.x = bat_transform.translation.x - 5.;
            bat_transform.scale.x = 0.175;

            hitbox_transform.translation = bat_transform.translation;
            hitbox_transform.translation.x = hitbox_transform.translation.x - 20.;
            hitbox_transform.translation.y = hitbox_transform.translation.y - 5.;
        }
        if unsafe { MOUSE_BUTTON_JUST_RELEASED } {
            for (mut ball, mut ball_velocity, mut ball_transform) in query.iter_mut() {
                let bat_to_ball_collision = bevy::sprite::collide_aabb::collide(
                    hitbox_transform.translation,
                    hitbox.size,
                    ball_transform.translation,
                    Vec2::new(BALL_SIZE, BALL_SIZE),
                );

                if (bat_to_ball_collision == Some(bevy::sprite::collide_aabb::Collision::Right))
                    || (bat_to_ball_collision == Some(bevy::sprite::collide_aabb::Collision::Left))
                    || (bat_to_ball_collision == Some(bevy::sprite::collide_aabb::Collision::Top))
                    || (bat_to_ball_collision
                        == Some(bevy::sprite::collide_aabb::Collision::Bottom))
                    || (bat_to_ball_collision
                        == Some(bevy::sprite::collide_aabb::Collision::Inside))
                {
                    ball_velocity.velocity = Vec3::splat(0.);
                    let change_x =
                        (((mouse_position.x - WIN_W) / 2.) - ball_transform.translation.x).abs();
                    let change_y =
                        ((-(mouse_position.y - WIN_H) / 2.) - ball_transform.translation.y).abs();
                    let mut new_velocity = Vec3::new(change_x, change_y, 0.);
                    new_velocity = new_velocity.normalize_or_zero();

                    if ((mouse_position.x - WIN_W) / 2.) > ball_transform.translation.x {
                        new_velocity.x = new_velocity.x;
                    } else {
                        new_velocity.x = -1. * new_velocity.x;
                    }

                    if (-(mouse_position.y - WIN_H) / 2.) > ball_transform.translation.y {
                        new_velocity.y = new_velocity.y;
                    } else {
                        new_velocity.y = -1. * new_velocity.y;
                    }

                    new_velocity.x = new_velocity.x * 100.;
                    new_velocity.y = new_velocity.y * 100.;

                    // if Q is pressed, backspin -> ball moves slower
                    if input.pressed(KeyCode::Q) {
                        new_velocity *= 0.5;
                    }

                    // if E is pressed, topspin -> ball moves faster
                    if input.pressed(KeyCode::E) {
                        new_velocity *= 1.5;
                    }

                    ball_velocity.velocity = new_velocity * ball.elasticity;
                }

                // let ball_position = ball_velocity.velocity.truncate();
                // println!("Ball position: {:?}", ball_position);

                /*let direction =  MOUSE_POSITION - ball_velocity.velocity.truncate();;
                println!("Direction: {:?}", direction);


                // Normalize the direction and set the ball's velocity
                let normalized_direction = direction.normalize_or_zero();
                //println!("Normalized direction: {:?}", normalized_direction);

                ball_velocity.velocity = Vec3::new(
                    normalized_direction.x * HIT_POWER.x,
                    normalized_direction.y * HIT_POWER.y,
                    0.0,
                );
                println!("Ball velocity: {:?}", ball_velocity.velocity);*/
            }

            // Reset the flags for the next interaction
            unsafe {
                MOUSE_BUTTON_JUST_RELEASED = false;
                BAT_TRANSFORMED = false;
            }
        }
    }
}

fn aim_follows_cursor(
    mut query_aim: Query<&mut Transform, With<Aim>>,
    //cursor_events: Res<Events<CursorMoved>>,
    window: Query<&Window>,
) {
    let mut aim_transform = query_aim.single_mut();
    /*let mut cursor_event_reader = cursor_events.get_reader();

    for event in cursor_event_reader.iter(&cursor_events) {
        // Update the aim's position to follow the cursor
        for mut aim_transform in query_aim.iter_mut() {
            aim_transform.translation.x = event.position.x - WIN_W / 2.0;
            aim_transform.translation.y = -(event.position.y - WIN_H / 2.0 + 40.0);
        }
    }*/

    if let Some(mouse_position) = window.single().physical_cursor_position() {
        aim_transform.translation.x = (mouse_position.x - WIN_W) / 2.;
        aim_transform.translation.y = -(mouse_position.y - WIN_H) / 2. - 40.;
    }
}
