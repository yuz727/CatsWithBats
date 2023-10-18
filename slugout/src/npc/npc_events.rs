use bevy::{math::vec2, prelude::*, sprite};

use super::npc::{NPCBat, NPCVelocity, States, NPC};
use crate::components::*;

const PLAYER_SIZE: f32 = 30.;
// 5px/frame @60Hz == 300px/s
const PLAYER_SPEED: f32 = 300.;
// 1px/frame^2 @60Hz == 3600px/s^2
const ACCEL_RATE: f32 = 3600.;

// Size Dimension for Objects
const RECLINER_WIDTH: f32 = 158.;
const RECLINER_HEIGHT: f32 = 178.;
const SIDETABLE_WIDTH: f32 = 125.;
const SIDETABLE_HEIGHT: f32 = 113.;
const TVSTAND_WIDTH: f32 = 160.;
const TVSTAND_HEIGHT: f32 = 160.;

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
            //info!("Chasing Player");
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

            if velocity.xlock == 0 {
                npc_transform.translation.x = (npc_transform.translation.x + change.x).clamp(
                    -(1280. / 2.) + PLAYER_SIZE / 2.,
                    1280. / 2. - PLAYER_SIZE / 2.,
                );
            }
            if velocity.ylock == 0 {
                npc_transform.translation.y = (npc_transform.translation.y + change.y).clamp(
                    -(720. / 2.) + PLAYER_SIZE / 2.,
                    720. / 2. - PLAYER_SIZE / 2.,
                );
            }

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
            //info!("Chasing Ball");
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
            if velocity.xlock == 0 {
                npc_transform.translation.x = (npc_transform.translation.x + change.x).clamp(
                    -(1280. / 2.) + PLAYER_SIZE / 2.,
                    1280. / 2. - PLAYER_SIZE / 2.,
                );
            }
            if velocity.ylock == 0 {
                npc_transform.translation.y = (npc_transform.translation.y + change.y).clamp(
                    -(720. / 2.) + PLAYER_SIZE / 2.,
                    720. / 2. - PLAYER_SIZE / 2.,
                );
            }

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
            //info!("Running Away from Ball");
            let ball_transform = ball.single_mut();
            //npc_transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
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

            if velocity.xlock == 0 {
                npc_transform.translation.x = (npc_transform.translation.x + change.x).clamp(
                    -(1280. / 2.) + PLAYER_SIZE / 2.,
                    1280. / 2. - PLAYER_SIZE / 2.,
                );
            }
            if velocity.ylock == 0 {
                npc_transform.translation.y = (npc_transform.translation.y + change.y).clamp(
                    -(720. / 2.) + PLAYER_SIZE / 2.,
                    720. / 2. - PLAYER_SIZE / 2.,
                );
            }
            bat_transform.translation.x = npc_transform.translation.x - 5.;
            bat_transform.translation.y = npc_transform.translation.y;
        }
    }
}

pub fn AvoidCollision(
    mut npcs: Query<
        (&Transform, &mut NPCVelocity),
        (
            With<NPC>,
            Without<Ball>,
            Without<Player>,
            Without<NPCBat>,
            Without<Object>,
        ),
    >,
    mut objects: Query<
        &Transform,
        (
            With<Object>,
            Without<Ball>,
            Without<Player>,
            Without<NPCBat>,
            Without<NPC>,
        ),
    >,
) {
    let npc_dimensions = vec2(PLAYER_SIZE, PLAYER_SIZE);
    // Iterate over combinations of possible objects that NPC can possibly collide with
    for (npc_transform, mut velocity) in npcs.iter_mut() {
        for obj_transform in objects.iter_mut() {
            let collision_result = bevy::sprite::collide_aabb::collide(
                npc_transform.translation,
                npc_dimensions,
                obj_transform.translation,
                // TODO: change detection depending on the obj  ect
                vec2(TVSTAND_WIDTH, TVSTAND_HEIGHT),
            );
            // If it is going to collide on the x-axis, lock x-axis movement
            if collision_result == Some(bevy::sprite::collide_aabb::Collision::Left)
                || collision_result == Some(bevy::sprite::collide_aabb::Collision::Right)
            {
                velocity.lock_x();
                velocity.unlock_y();
                return;
            // If it is going to collide on the y-axis, lock y-axis movement
            } else if collision_result == Some(bevy::sprite::collide_aabb::Collision::Top)
                || collision_result == Some(bevy::sprite::collide_aabb::Collision::Bottom)
            {
                velocity.lock_y();
                velocity.unlock_x();
                return;
            }
            velocity.unlock_x();
            velocity.unlock_y();
        }
    }
}
