use bevy::prelude::*;

use super::npc::{NPCBat, NPCVelocity, States, NPC};
use crate::components::*;

const PLAYER_SIZE: f32 = 30.;
// 5px/frame @60Hz == 300px/s
const PLAYER_SPEED: f32 = 300.;
// 1px/frame^2 @60Hz == 3600px/s^2
const ACCEL_RATE: f32 = 3600.;

// Just go the the player straight
pub fn approach_player(
    mut npcs: Query<
        (&mut Transform, &mut NPCVelocity, &States),
        (With<NPC>, Without<Ball>, Without<Player>, Without<NPCBat>),
    >,
    mut bat: Query<&mut Transform, (With<NPCBat>, Without<Player>, Without<NPC>, Without<Ball>)>,
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    for (mut npc_transform, mut velocity, state) in npcs.iter_mut() {
        if matches!(state, States::AggressionPlayer) {
            let mut bat_transform = bat.single_mut();
            info!("Chasing Player");
            //debug!("Chasing Player");
            let player_transform = player.single_mut();

            let mut deltav = Vec2::splat(0.);
            if npc_transform.translation.x < player_transform.translation.x {
                deltav.x += 10.;
            }
            if npc_transform.translation.x > player_transform.translation.x {
                deltav.x -= 10.;
            }
            if npc_transform.translation.y < player_transform.translation.y {
                deltav.y += 10.;
            }
            if npc_transform.translation.y > player_transform.translation.y {
                deltav.y -= 10.;
            }

            let deltat = time.delta_seconds();
            let acc = ACCEL_RATE * deltat;
            velocity.velocity = if deltav.length() > 0. {
                (velocity.velocity + (deltav.normalize_or_zero() * acc))
                    .clamp_length_max(PLAYER_SPEED)
            } else if velocity.velocity.length() > acc {
                velocity.velocity + (velocity.velocity.normalize_or_zero() * -acc)
            } else {
                Vec2::splat(0.)
            };
            let change = velocity.velocity * deltat;

            npc_transform.translation.x = (npc_transform.translation.x + change.x).clamp(
                -(1280. / 2.) + PLAYER_SIZE / 2.,
                1280. / 2. - PLAYER_SIZE / 2.,
            );
            npc_transform.translation.y = (npc_transform.translation.y + change.y).clamp(
                -(720. / 2.) + PLAYER_SIZE / 2.,
                720. / 2. - PLAYER_SIZE / 2.,
            );

            bat_transform.translation.x = npc_transform.translation.x - 5.;
            bat_transform.translation.y = npc_transform.translation.y;
        }
    }
}

pub fn approach_ball(
    mut npcs: Query<
        (&mut Transform, &mut NPCVelocity, &States),
        (With<NPC>, Without<Ball>, Without<Player>, Without<NPCBat>),
    >,
    mut bat: Query<&mut Transform, (With<NPCBat>, Without<Player>, Without<NPC>, Without<Ball>)>,
    mut ball: Query<&mut Transform, (With<Ball>, Without<NPC>, Without<Player>, Without<NPCBat>)>,
    time: Res<Time>,
) {
    for (mut npc_transform, mut velocity, state) in npcs.iter_mut() {
        if matches!(state, States::AggressionBall) {
            let mut bat_transform = bat.single_mut();
            info!("Chasing Ball");
            let ball_transform = ball.single_mut();
            npc_transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
            let mut deltav = Vec2::splat(0.);
            if npc_transform.translation.x < ball_transform.translation.x {
                deltav.x += 10.;
            }
            if npc_transform.translation.x > ball_transform.translation.x {
                deltav.x -= 10.;
            }
            if npc_transform.translation.y < ball_transform.translation.y {
                deltav.y += 10.;
            }
            if npc_transform.translation.y > ball_transform.translation.y {
                deltav.y -= 10.;
            }

            let deltat = time.delta_seconds();
            let acc = ACCEL_RATE * deltat;
            velocity.velocity = if deltav.length() > 0. {
                (velocity.velocity + (deltav.normalize_or_zero() * acc))
                    .clamp_length_max(PLAYER_SPEED)
            } else if velocity.velocity.length() > acc {
                velocity.velocity + (velocity.velocity.normalize_or_zero() * -acc)
            } else {
                Vec2::splat(0.)
            };
            let change = velocity.velocity * deltat;

            npc_transform.translation.x = (npc_transform.translation.x + change.x).clamp(
                -(1280. / 2.) + PLAYER_SIZE / 2.,
                1280. / 2. - PLAYER_SIZE / 2.,
            );
            npc_transform.translation.y = (npc_transform.translation.y + change.y).clamp(
                -(720. / 2.) + PLAYER_SIZE / 2.,
                720. / 2. - PLAYER_SIZE / 2.,
            );
            bat_transform.translation.x = npc_transform.translation.x - 5.;
            bat_transform.translation.y = npc_transform.translation.y;
        }
    }
}

pub fn evade_ball(
    mut npcs: Query<
        (&mut Transform, &mut NPCVelocity, &States),
        (With<NPC>, Without<Ball>, Without<Player>, Without<NPCBat>),
    >,
    mut bat: Query<&mut Transform, (With<NPCBat>, Without<Player>, Without<NPC>, Without<Ball>)>,
    mut ball: Query<&mut Transform, (With<Ball>, Without<NPC>, Without<Player>, Without<NPCBat>)>,
    time: Res<Time>,
) {
    for (mut npc_transform, mut velocity, state) in npcs.iter_mut() {
        if matches!(state, States::Evade) {
            let mut bat_transform = bat.single_mut();
            info!("Running Away from Ball");
            let ball_transform = ball.single_mut();
            npc_transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
            let mut deltav = Vec2::splat(0.);
            if npc_transform.translation.x < ball_transform.translation.x {
                deltav.x -= 10.;
            }
            if npc_transform.translation.x > ball_transform.translation.x {
                deltav.x += 10.;
            }
            if npc_transform.translation.y < ball_transform.translation.y {
                deltav.y -= 10.;
            }
            if npc_transform.translation.y > ball_transform.translation.y {
                deltav.y += 10.;
            }

            let deltat = time.delta_seconds();
            let acc = ACCEL_RATE * deltat;
            velocity.velocity = if deltav.length() > 0. {
                (velocity.velocity + (deltav.normalize_or_zero() * acc))
                    .clamp_length_max(PLAYER_SPEED)
            } else if velocity.velocity.length() > acc {
                velocity.velocity + (velocity.velocity.normalize_or_zero() * -acc)
            } else {
                Vec2::splat(0.)
            };
            let change = velocity.velocity * deltat;

            npc_transform.translation.x = (npc_transform.translation.x + change.x).clamp(
                -(1280. / 2.) + PLAYER_SIZE / 2.,
                1280. / 2. - PLAYER_SIZE / 2.,
            );
            npc_transform.translation.y = (npc_transform.translation.y + change.y).clamp(
                -(720. / 2.) + PLAYER_SIZE / 2.,
                720. / 2. - PLAYER_SIZE / 2.,
            );
            bat_transform.translation.x = npc_transform.translation.x - 5.;
            bat_transform.translation.y = npc_transform.translation.y;
        }
    }
}
