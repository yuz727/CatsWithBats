use bevy::prelude::*;
// use serde::{Deserialize, Serialize};

use crate::game::player_movement::PlayerVelocity;
// use crate::multiplayer::{ClientSocket, SocketAddress};
use crate::GameState;

use super::components::{Bat, Hitbox, Player, PowerUp};
use super::npc::{NPCBat, BIG_BAT_PU, NPC};

const PLAYER_SIZE: f32 = 30.; // size of power up as well
                              // 5px/frame @60Hz == 300px/s

// 1px/frame^2 @60Hz == 3600px/s^2
//const ACCEL_RATE: f32 = 58000.;

pub struct PowerUpPlugin;

impl Plugin for PowerUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn_powerups);
        app.add_systems(Update, player_powerups.run_if(in_state(GameState::Game)));
        app.add_systems(
            Update,
            apply_powerups
                .run_if(in_state(GameState::Game))
                .after(player_powerups),
        );
    }
}

pub fn spawn_powerups(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(240, 140, 100),
                custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(80., -80., 10.),
            ..Default::default()
        })
        .insert(PowerUp {
            powerup: "bigbat".to_string(),
            active: true,
        });

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(100, 240, 100),
                custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(220., 130., 10.),
            ..Default::default()
        })
        .insert(PowerUp {
            powerup: "confusion".to_string(),
            active: true,
        });

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(100, 100, 240),
                custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                ..Default::default()
            },
            transform: Transform::from_xyz(-200., -230., 10.),
            ..Default::default()
        })
        .insert(PowerUp {
            powerup: "invisibility".to_string(),
            active: true,
        });
}

pub fn player_powerups(
    mut player_query: Query<
        (&mut Transform, &mut Player),
        (With<Player>, Without<PowerUp>, Without<NPC>),
    >,
    mut powerups: Query<
        (&mut Transform, &mut PowerUp, &mut Visibility),
        (With<PowerUp>, Without<Player>, Without<NPC>),
    >,
    mut npc_query: Query<(&Transform, &mut NPC), (With<NPC>, Without<PowerUp>, Without<Player>)>,
) {
    let (player_transform, mut player) = player_query.single_mut();
    for (npc_transform, mut npc) in npc_query.iter_mut() {
        for (powerup_transform, mut powerup, mut powerup_visibility) in powerups.iter_mut() {
            let player_collision = bevy::sprite::collide_aabb::collide(
                player_transform.translation,
                Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
                powerup_transform.translation,
                Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
            );

            let npc_collision = bevy::sprite::collide_aabb::collide(
                npc_transform.translation,
                Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
                powerup_transform.translation,
                Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
            );
            if npc_collision == Some(bevy::sprite::collide_aabb::Collision::Left)
                || npc_collision == Some(bevy::sprite::collide_aabb::Collision::Right)
                || npc_collision == Some(bevy::sprite::collide_aabb::Collision::Top)
                || npc_collision == Some(bevy::sprite::collide_aabb::Collision::Bottom)
            {
                if npc.powerup == "none".to_string() {
                    npc.powerup = powerup.powerup.clone();
                    npc.powerup_timer = 10.;
                    powerup.active = false;
                    *powerup_visibility = Visibility::Hidden;
                }
            }
            if powerup.active == true {
                if player_collision == Some(bevy::sprite::collide_aabb::Collision::Left)
                    || player_collision == Some(bevy::sprite::collide_aabb::Collision::Right)
                    || player_collision == Some(bevy::sprite::collide_aabb::Collision::Top)
                    || player_collision == Some(bevy::sprite::collide_aabb::Collision::Bottom)
                {
                    //println!("running");
                    if player.powerup == "none".to_string() {
                        player.powerup = powerup.powerup.clone();
                        player.powerup_timer = 10.;
                        powerup.active = false; // Set the flag
                        *powerup_visibility = Visibility::Hidden;
                    }
                }
            }
        }
    }
}

pub fn apply_powerups(
    time: Res<Time>,
    mut player_query: Query<
        &mut Player,
        (
            With<Player>,
            Without<PowerUp>,
            Without<Hitbox>,
            Without<Bat>,
        ),
    >,
    mut player_visibility: Query<
        &mut Visibility,
        (
            With<Player>,
            Without<PowerUp>,
            Without<Hitbox>,
            Without<Bat>,
        ),
    >,
    mut bat: Query<
        &mut Transform,
        (
            With<Bat>,
            Without<PowerUp>,
            Without<Player>,
            Without<Hitbox>,
            Without<NPC>,
        ),
    >,
    mut powerups: Query<
        (&mut Transform, &mut PowerUp, &mut Visibility),
        (With<PowerUp>, Without<Player>, Without<NPC>),
    >,
    mut bat_visibility: Query<
        &mut Visibility,
        (
            With<Bat>,
            Without<PowerUp>,
            Without<Hitbox>,
            Without<Player>,
        ),
    >,
    mut npc_query: Query<
        (&mut Transform, &mut NPC),
        (With<NPC>, Without<Player>, Without<Bat>, Without<Hitbox>),
    >,
    mut npc_visibility: Query<
        &mut Visibility,
        (
            With<NPC>,
            Without<PowerUp>,
            Without<Player>,
            Without<Bat>,
            Without<Hitbox>,
        ),
    >,
    mut npc_bat_visibility: Query<
        &mut Visibility,
        (
            With<NPCBat>,
            Without<NPC>,
            Without<PowerUp>,
            Without<Player>,
            Without<Bat>,
            Without<Hitbox>,
        ),
    >,
    mut npc_bat: Query<
        &mut Transform,
        (
            With<NPCBat>,
            Without<NPC>,
            Without<PowerUp>,
            Without<Player>,
            Without<Bat>,
            Without<Hitbox>,
        ),
    >,
    mut hitbox_query: Query<
        (&mut Transform, &mut Hitbox),
        (
            With<Hitbox>,
            Without<PowerUp>,
            Without<Player>,
            Without<Bat>,
        ),
    >,
    mut player_velocity_query: Query<
        &mut PlayerVelocity,
        (
            With<PlayerVelocity>,
            Without<PowerUp>,
            Without<Hitbox>,
            Without<Bat>,
        ),
    >,
) {
    let mut player = player_query.single_mut();
    let mut bat_transform = bat.single_mut();
    let mut npc_bat_transform = npc_bat.single_mut();
    let (mut hitbox_transform, mut hitbox) = hitbox_query.single_mut();
    let mut player_velocity = player_velocity_query.single_mut();
    let mut player_visibility = player_visibility.single_mut();
    let mut bat_visibility = bat_visibility.single_mut();
    let mut npc_visibility = npc_visibility.single_mut();
    let mut npc_bat_visibility = npc_bat_visibility.single_mut();

    for (mut _npc_transform, mut npc) in npc_query.iter_mut() {
        for (_powerup_transform, mut powerup, _powerup_visibility) in powerups.iter_mut() {
            //Big Bat Powerup for Player
            if player.powerup == "bigbat".to_string() && powerup.active == true {
                if player.powerup_timer == 10. {
                    *bat_transform = bat_transform.with_scale(Vec3::new(0.3, 0.3, 0.));
                    *hitbox_transform = hitbox_transform.with_scale(Vec3::new(1.75, 1.75, 0.));
                    hitbox.size = Vec2::new(78.75, 131.25);
                    powerup.active = false;
                }
                player.powerup_timer = player.powerup_timer - time.delta_seconds();
                if player.powerup_timer <= 0. {
                    player.powerup = "none".to_string();
                    powerup.active = false;
                    *bat_transform = bat_transform.with_scale(Vec3::new(0.175, 0.175, 0.));
                    *hitbox_transform = hitbox_transform.with_scale(Vec3::new(1., 1., 0.));
                    hitbox.size = Vec2::new(45., 75.);
                }
            }
            //Big Bat Powerup for NPC
            if npc.powerup == "bigbat".to_string() && powerup.active == true {
                if npc.powerup_timer == 10. {
                    *npc_bat_transform = npc_bat_transform.with_scale(Vec3::new(0.3, 0.3, 0.));
                    powerup.active = false;
                    unsafe { BIG_BAT_PU = true };
                }
                npc.powerup_timer -= time.delta_seconds();
                if npc.powerup_timer <= 0. {
                    npc.powerup = "none".to_string();
                    powerup.active = false;
                    *npc_bat_transform = npc_bat_transform.with_scale(Vec3::new(0.13, 0.13, 0.));
                    unsafe { BIG_BAT_PU = false };
                }
            }
            //Invisibility Power Up for Player
            if player.powerup == "invisibility".to_string() && powerup.active == true {
                if player.powerup_timer == 10. {
                    *player_visibility = Visibility::Hidden;
                    *bat_visibility = Visibility::Hidden;
                    powerup.active = false;
                }
                player.powerup_timer = player.powerup_timer - time.delta_seconds();

                if player.powerup_timer <= 0. {
                    player.powerup = "none".to_string();
                    powerup.active = false;
                    *player_visibility = Visibility::Visible;
                    *bat_visibility = Visibility::Visible;
                }
            }
            //Invisibility Power Up for NPC
            if npc.powerup == "invisibility".to_string() && powerup.active == true {
                if npc.powerup_timer == 10. {
                    *npc_visibility = Visibility::Hidden;
                    *npc_bat_visibility = Visibility::Hidden;
                    powerup.active = false;
                }
                npc.powerup_timer = npc.powerup_timer - time.delta_seconds();

                if npc.powerup_timer <= 0. {
                    npc.powerup = "none".to_string();
                    powerup.active = false;
                    *npc_visibility = Visibility::Visible;
                    *npc_bat_visibility = Visibility::Visible;
                }
            }
            //Confusion Powerup for NPC
            if npc.powerup == "confusion".to_string() && powerup.active == true {
                if npc.powerup_timer == 10. {
                    npc.confused = true;
                    powerup.active = false;
                }
                npc.powerup_timer = npc.powerup_timer - time.delta_seconds();

                if npc.powerup_timer <= 0. {
                    npc.powerup = "none".to_string();
                    npc.confused = false;
                    powerup.active = false;
                }
            }
            //Confusion Powerup for Player
            if player.powerup == "confusion".to_string() && powerup.active == true {
                if player.powerup_timer == 10. {
                    player_velocity.confused = true;
                    powerup.active = false;
                }
                player.powerup_timer = player.powerup_timer - time.delta_seconds();

                if player.powerup_timer <= 0. {
                    player.powerup = "none".to_string();
                    player_velocity.confused = false;
                    powerup.active = false;
                }
            }
        }
    }
}
