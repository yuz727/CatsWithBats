use bevy::{math::vec2, prelude::*};

use super::npc::{NPCBat, NPCFace, NPCTimer, NPCVelocity, States, NPC};
use crate::components::*;

const NPC_SIZE: f32 = 30.;
// 5px/frame @60Hz == 300px/s
const NPC_SPEED: f32 = 300.;
// 1px/frame^2 @60Hz == 3600px/s^2
const NPC_ACCEL_RATE: f32 = 18000.;

const HIT_POWER: Vec3 = Vec3::new(500.0, 500.0, 2.0);

// Just go the the player straight
pub fn approach_player(
    mut npcs: Query<
        (&mut Transform, &mut NPCVelocity, &States),
        (With<NPC>, Without<Ball>, Without<Player>, Without<NPCBat>),
    >,
    mut bat: Query<
        &mut Transform,
        (
            With<NPCBat>,
            Without<Player>,
            Without<NPC>,
            Without<Ball>,
            Without<NPCFace>,
        ),
    >,
    mut face: Query<
        &mut Transform,
        (
            With<NPCFace>,
            Without<Player>,
            Without<NPC>,
            Without<Ball>,
            Without<NPCBat>,
        ),
    >,
    player: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    for (mut npc_transform, mut velocity, state) in npcs.iter_mut() {
        if matches!(state, States::AggressionPlayer) {
            for mut bat_transform in bat.iter_mut() {
                //info!("Chasing Player");
                //debug!("Chasing Player");
                for player_transform in player.iter() {
                    for mut face_transform in face.iter_mut() {
                        let mut deltav = Vec2::splat(0.);
                        if npc_transform.translation.x < player_transform.translation.x {
                            deltav.x += 1000.;
                        }
                        if npc_transform.translation.x > player_transform.translation.x {
                            deltav.x -= 1000.;
                        }
                        if npc_transform.translation.y < player_transform.translation.y {
                            deltav.y += 1000.;
                        }
                        if npc_transform.translation.y > player_transform.translation.y {
                            deltav.y -= 1000.;
                        }

                        let deltat = time.delta_seconds();
                        let acc = NPC_ACCEL_RATE * deltat;
                        velocity.velocity = if deltav.length() > 0. {
                            (velocity.velocity + (deltav.normalize_or_zero() * acc))
                                .clamp_length_max(NPC_SPEED)
                        } else if velocity.velocity.length() > acc {
                            velocity.velocity + (velocity.velocity.normalize_or_zero() * -acc)
                        } else {
                            Vec2::splat(0.)
                        };
                        velocity.velocity = velocity.velocity * deltat;
                        if velocity.xlock == 0 {
                            npc_transform.translation.x = (npc_transform.translation.x
                                + velocity.velocity.x)
                                .clamp(-(1280. / 2.) + NPC_SIZE / 2., 1280. / 2. - NPC_SIZE / 2.);
                        }
                        if velocity.ylock == 0 {
                            npc_transform.translation.y = (npc_transform.translation.y
                                + velocity.velocity.y)
                                .clamp(-(720. / 2.) + NPC_SIZE / 2., 720. / 2. - NPC_SIZE / 2.);
                        }

                        bat_transform.translation.x = npc_transform.translation.x - 5.;
                        bat_transform.translation.y = npc_transform.translation.y;
                        face_transform.translation.x = npc_transform.translation.x;
                        face_transform.translation.y = npc_transform.translation.y;
                    }
                }
            }
        }
    }
}

pub fn approach_ball(
    mut npcs: Query<
        (&mut Transform, &mut NPCVelocity, &States),
        (
            With<NPC>,
            Without<Ball>,
            Without<Player>,
            Without<NPCBat>,
            Without<NPCFace>,
        ),
    >,
    mut bat: Query<
        &mut Transform,
        (
            With<NPCBat>,
            Without<Player>,
            Without<NPC>,
            Without<Ball>,
            Without<NPCFace>,
        ),
    >,
    mut face: Query<
        &mut Transform,
        (
            With<NPCFace>,
            Without<Player>,
            Without<NPC>,
            Without<Ball>,
            Without<NPCBat>,
        ),
    >,
    ball: Query<&Transform, (With<Ball>, Without<NPC>, Without<Player>, Without<NPCBat>)>,
    time: Res<Time>,
) {
    for (mut npc_transform, mut velocity, state) in npcs.iter_mut() {
        if matches!(state, States::AggressionBall) {
            for mut bat_transform in bat.iter_mut() {
                //info!("Chasing Ball");
                for ball_transform in ball.iter() {
                    for mut face_transform in face.iter_mut() {
                        npc_transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
                        let mut deltav = Vec2::splat(0.);
                        if npc_transform.translation.x < ball_transform.translation.x {
                            deltav.x += 1000.;
                        }
                        if npc_transform.translation.x > ball_transform.translation.x {
                            deltav.x -= 1000.;
                        }
                        if npc_transform.translation.y < ball_transform.translation.y {
                            deltav.y += 1000.;
                        }
                        if npc_transform.translation.y > ball_transform.translation.y {
                            deltav.y -= 1000.;
                        }

                        let deltat = time.delta_seconds();
                        let acc = NPC_ACCEL_RATE * deltat;
                        velocity.velocity = if deltav.length() > 0. {
                            (velocity.velocity + (deltav.normalize_or_zero() * acc))
                                .clamp_length_max(NPC_SPEED)
                        } else if velocity.velocity.length() > acc {
                            velocity.velocity + (velocity.velocity.normalize_or_zero() * -acc)
                        } else {
                            Vec2::splat(0.)
                        };
                        velocity.velocity = velocity.velocity * deltat;
                        if velocity.xlock == 0 {
                            npc_transform.translation.x = (npc_transform.translation.x
                                + velocity.velocity.x)
                                .clamp(-(1280. / 2.) + NPC_SIZE / 2., 1280. / 2. - NPC_SIZE / 2.);
                        }
                        if velocity.ylock == 0 {
                            npc_transform.translation.y = (npc_transform.translation.y
                                + velocity.velocity.y)
                                .clamp(-(720. / 2.) + NPC_SIZE / 2., 720. / 2. - NPC_SIZE / 2.);
                        }

                        bat_transform.translation.x = npc_transform.translation.x - 5.;
                        bat_transform.translation.y = npc_transform.translation.y;
                        face_transform.translation.x = npc_transform.translation.x;
                        face_transform.translation.y = npc_transform.translation.y;
                    }
                }
            }
        }
    }
}

pub fn evade_ball(
    mut npcs: Query<
        (&mut Transform, &mut NPCVelocity, &States),
        (With<NPC>, Without<Ball>, Without<Player>, Without<NPCBat>),
    >,
    mut bat: Query<
        &mut Transform,
        (
            With<NPCBat>,
            Without<Player>,
            Without<NPC>,
            Without<Ball>,
            Without<NPCFace>,
        ),
    >,
    mut face: Query<
        &mut Transform,
        (
            With<NPCFace>,
            Without<Player>,
            Without<NPC>,
            Without<Ball>,
            Without<NPCBat>,
        ),
    >,
    ball: Query<&Transform, (With<Ball>, Without<NPC>, Without<Player>, Without<NPCBat>)>,
    time: Res<Time>,
) {
    let mut velocity_change = 0;
    for (mut npc_transform, mut velocity, state) in npcs.iter_mut() {
        if matches!(state, States::Evade) {
            for mut bat_transform in bat.iter_mut() {
                //info!("Running Away from Ball");
                for ball_transform in ball.iter() {
                    for mut face_transform in face.iter_mut() {
                        //npc_transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
                        let mut deltav = Vec2::splat(0.);
                        if npc_transform.translation.x < ball_transform.translation.x {
                            deltav.x -= 1000.;
                        }
                        if npc_transform.translation.x > ball_transform.translation.x {
                            deltav.x += 1000.;
                        }
                        if npc_transform.translation.y < ball_transform.translation.y {
                            deltav.y -= 1000.;
                        }
                        if npc_transform.translation.y > ball_transform.translation.y {
                            deltav.y += 1000.;
                        }

                        let deltat = time.delta_seconds();
                        let acc = NPC_ACCEL_RATE * deltat;
                        velocity.velocity = if deltav.length() > 0. {
                            (velocity.velocity + (deltav.normalize_or_zero() * acc))
                                .clamp_length_max(NPC_SPEED)
                        } else if velocity.velocity.length() > acc {
                            velocity.velocity + (velocity.velocity.normalize_or_zero() * -acc)
                        } else {
                            Vec2::splat(0.)
                        };
                        velocity.velocity = velocity.velocity * deltat;

                        if velocity.xlock == 0 {
                            npc_transform.translation.x = (npc_transform.translation.x
                                + velocity.velocity.x)
                                .clamp(-(1280. / 2.) + NPC_SIZE / 2., 1280. / 2. - NPC_SIZE / 2.);
                        }
                        if velocity.ylock == 0 {
                            npc_transform.translation.y = (npc_transform.translation.y
                                + velocity.velocity.y)
                                .clamp(-(720. / 2.) + NPC_SIZE / 2., 720. / 2. - NPC_SIZE / 2.);
                        }
                        bat_transform.translation.x = npc_transform.translation.x - 5.;
                        bat_transform.translation.y = npc_transform.translation.y;
                        face_transform.translation.x = npc_transform.translation.x;
                        face_transform.translation.y = npc_transform.translation.y;
                    }
                }
            }
        }
    }
}

pub fn avoid_collision(
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
) {
    let npc_dimensions = vec2(NPC_SIZE, NPC_SIZE);
    // Iterate over combinations of possible objects that NPC can possibly collide with
    for (npc_transform, mut velocity) in npcs.iter_mut() {
        let recliner_collision = bevy::sprite::collide_aabb::collide(
            npc_transform.translation,
            npc_dimensions,
            Vec3::new(-60., 210., 1.),
            Vec2::new(109., 184.),
        );
        let tv_stand_collision = bevy::sprite::collide_aabb::collide(
            npc_transform.translation,
            npc_dimensions,
            Vec3::new(0., -250., 1.),
            Vec2::new(164., 103.),
        );
        let side_table_collision = bevy::sprite::collide_aabb::collide(
            npc_transform.translation,
            npc_dimensions,
            Vec3::new(120., 170., 1.),
            Vec2::new(103., 107.),
        );

        // If it is going to collide on the x-axis, lock x-axis movement
        if recliner_collision == Some(bevy::sprite::collide_aabb::Collision::Left)
            || recliner_collision == Some(bevy::sprite::collide_aabb::Collision::Right)
            || tv_stand_collision == Some(bevy::sprite::collide_aabb::Collision::Left)
            || tv_stand_collision == Some(bevy::sprite::collide_aabb::Collision::Right)
            || side_table_collision == Some(bevy::sprite::collide_aabb::Collision::Left)
            || side_table_collision == Some(bevy::sprite::collide_aabb::Collision::Right)
        {
            velocity.lock_x();
            velocity.unlock_y();
            return;
        // If it is going to collide on the y-axis, lock y-axis movement
        } else if recliner_collision == Some(bevy::sprite::collide_aabb::Collision::Top)
            || recliner_collision == Some(bevy::sprite::collide_aabb::Collision::Bottom)
            || tv_stand_collision == Some(bevy::sprite::collide_aabb::Collision::Top)
            || tv_stand_collision == Some(bevy::sprite::collide_aabb::Collision::Bottom)
            || side_table_collision == Some(bevy::sprite::collide_aabb::Collision::Top)
            || side_table_collision == Some(bevy::sprite::collide_aabb::Collision::Bottom)
        {
            velocity.lock_y();
            velocity.unlock_x();
            return;
        }
        velocity.unlock_x();
        velocity.unlock_y();
    }
}

pub fn bat_swing(
    mut npcs: Query<
        (&NPCVelocity, &States),
        (With<NPC>, Without<Ball>, Without<Player>, Without<NPCBat>),
    >,
    mut bat: Query<
        (&mut Transform, &mut NPCTimer),
        (With<NPCBat>, Without<Player>, Without<NPC>, Without<Ball>),
    >,
    mut ball: Query<
        &mut BallVelocity,
        (With<Ball>, Without<NPC>, Without<Player>, Without<NPCBat>),
    >,
    time: Res<Time>,
) {
    for (npc_velocity, state) in npcs.iter_mut() {
        if matches!(state, States::Idle) {
            // bat swing animation
            let (mut bat_transform, mut timer) = bat.single_mut();

            // Not really working, will fix the animation later
            timer.tick(time.delta());
            if timer.just_finished() {
                bat_transform.scale.y = -0.13;
                //     bat_transform.scale.y = 0.13;
            }
            bat_transform.scale.y = 0.13;
            for mut ball_velocity in ball.iter_mut() {
                // Initialize the ball's velocity
                ball_velocity.velocity = Vec3::new(0.0, 0.0, 0.0);

                // hit based on game pong functionality, until i can get the cursor library approved
                if npc_velocity.velocity.y > 0. {
                    ball_velocity.velocity.y = HIT_POWER.y; //ball moves up
                }
                if npc_velocity.velocity.y < 0. {
                    ball_velocity.velocity.y = -HIT_POWER.y; //down
                }
                if npc_velocity.velocity.x < 0. {
                    ball_velocity.velocity.x = -HIT_POWER.x; //left
                }
                if npc_velocity.velocity.x > 0. {
                    ball_velocity.velocity.x = HIT_POWER.x; //right
                }
            }
        }
    }
}
