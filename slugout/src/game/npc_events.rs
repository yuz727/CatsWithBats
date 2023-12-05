/// Imports
use crate::game::npc::States;
use crate::game::npc::*;
use crate::game::pathfinding::*;
use crate::MAP;

use bevy::prelude::*;
use rand::prelude::*;

use super::components::Ball;
use super::components::BallVelocity;
use super::components::Player;

/// Constants for movement calculation
const NPC_SIZE: f32 = 30.;
const NPC_SPEED: f32 = 300.;
const NPC_ACCEL_RATE: f32 = 18000.;

/// Return whether a ball is going to hit the npc
pub fn danger_check(
    npc_translation: Vec3,
    time: &Res<Time>,
    ball_query: &Query<
        (&Transform, &BallVelocity, &Ball),
        (With<Ball>, Without<NPC>, Without<NPCBat>, Without<Player>),
    >,
) -> bool {
    // For every ball
    for (ball_transform, ball_velocity, ball) in ball_query.iter() {
        // If a ball is close enough (< 200 pixels away) and it is moving towards the npc, then return true
        let ball_future_position = Vec2::new(
            (ball_transform.translation.x + (ball_velocity.velocity.x * time.delta_seconds()))
                .clamp(-(1280. / 2.) + ball.radius, 1280. / 2. - ball.radius),
            (ball_transform.translation.y + (ball_velocity.velocity.y * time.delta_seconds()))
                .clamp(-(720. / 2.) + ball.radius, 720. / 2. - ball.radius),
        );
        if ball_transform.translation.distance(npc_translation) < 200.
            && ball_future_position.distance(npc_translation.truncate())
                < ball_transform.translation.distance(npc_translation)
        {
            return true;
        }
    }
    return false;
}

/// NPC movement in danger state, sidestep to avoid getting hit by ball
pub fn sidestep(
    mut npc: Query<
        (&mut Transform, &mut NPCVelocity, &mut States, &mut Path),
        (With<NPC>, Without<NPCBat>, Without<Player>),
    >,
    mut bat: Query<&mut Transform, (With<NPCBat>, Without<NPC>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<NPCBat>, Without<NPC>)>,
    time: Res<Time>,
) {
    for (mut npc_transform, mut velocity, state, path) in npc.iter_mut() {
        if state.is_danger() {
            for player_transform in player.iter() {
                let mut bat_transform = bat.single_mut();

                let mut deltav = Vec2::splat(0.);

                if path.ball.x < 0. && path.ball.y < 0. {
                    deltav.x -= 1000.;
                    deltav.y += 1000.;
                } else if path.ball.x < 0. && path.ball.y > 0. {
                    deltav.x += 1000.;
                    deltav.y += 1000.;
                } else if path.ball.x > 0. && path.ball.y < 0. {
                    deltav.x -= 1000.;
                    deltav.y -= 1000.;
                } else if path.ball.x > 0. && path.ball.y > 0. {
                    deltav.x += 1000.;
                    deltav.y -= 1000.;
                } else if path.ball.x < 0. {
                    deltav.y += 1000.;
                } else if path.ball.x > 0. {
                    deltav.y -= 1000.;
                } else if path.ball.y < 0. {
                    deltav.x -= 1000.;
                } else if path.ball.y > 0. {
                    deltav.x += 1000.;
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

                if unsafe { MAP == 1 } {
                    velocity.velocity = collision_check_map1(
                        npc_transform.translation,
                        velocity.velocity,
                        player_transform.translation,
                    );
                } else if unsafe { MAP == 2 || MAP == 3 } {
                    velocity.velocity = collision_check_no_objects(
                        npc_transform.translation,
                        velocity.velocity,
                        player_transform.translation,
                    );
                } else if unsafe { MAP == 4 } {
                    velocity.velocity = collision_check_map4(
                        npc_transform.translation,
                        velocity.velocity,
                        player_transform.translation,
                    );
                }
                velocity.velocity = velocity.velocity * deltat;

                velocity.velocity = velocity.velocity * deltat;
                npc_transform.translation.x = (npc_transform.translation.x + velocity.velocity.x)
                    .clamp(-(1280. / 2.) + NPC_SIZE / 2., 1280. / 2. - NPC_SIZE / 2.);
                npc_transform.translation.y = (npc_transform.translation.y + velocity.velocity.y)
                    .clamp(-(720. / 2.) + NPC_SIZE / 2., 720. / 2. - NPC_SIZE / 2.);

                // Fixes Misalign caused by the pathfinding grids being 4x4 pixel chunks

                npc_transform.translation.x = (npc_transform.translation.x + velocity.velocity.x)
                    .clamp(-(1280. / 2.) + NPC_SIZE / 2., 1280. / 2. - NPC_SIZE / 2.);
                npc_transform.translation.y = (npc_transform.translation.y + velocity.velocity.y)
                    .clamp(-(720. / 2.) + NPC_SIZE / 2., 720. / 2. - NPC_SIZE / 2.);

                bat_transform.translation.x = npc_transform.translation.x - 5.;
                bat_transform.translation.y = npc_transform.translation.y;
            } // for player iter
        } // if in danger
    } // for npc iter
}

/// Return whether player is < 200 pixels in distance to the NPC
pub fn player_proximity_check(npc_translation: Vec3, player_translation: Vec3) -> bool {
    // If player is close enough (< 200 pixels away) and it is moving towards the npc, then return true
    if player_translation.distance(npc_translation) < 200. {
        return true;
    }
    return false;
}

/// Return whether there is a current point for the npc to go to
pub fn tag_is_null(path: &Path) -> bool {
    // Implement TAG null check logic
    if path.goal.x == -1. && path.goal.y == -1. {
        return true;
    }
    return false;
}

/// Find the closest ball to NPC, and set the goal to it
pub fn set_tag_to_closest_ball(
    npc_translation: Vec3,
    path: &mut Path,
    ball_query: &Query<
        (&Transform, &BallVelocity, &Ball),
        (With<Ball>, Without<NPC>, Without<NPCBat>, Without<Player>),
    >,
) -> bool {
    let mut ret = Vec2::splat(10000000000.);
    let mut new_ball_vel = Vec2::splat(-1.);
    for (ball_transform, velocity, _) in ball_query.iter() {
        if ball_transform.translation.distance(npc_translation)
            < npc_translation.truncate().distance(ret)
        {
            ret = ball_transform.translation.truncate();
            new_ball_vel = velocity.velocity.truncate();
        }
    }
    if ret.x == 10000000000. && ret.y == 10000000000. {
        return false;
    }
    path.goal = ret;
    path.set_new_ball(new_ball_vel);
    return true;
}

/// Return a check on difficulty. Higher the Difficulty the more likely the check would pass
pub fn difficulty_check(difficulty: i32) -> bool {
    let mut rand = thread_rng();
    if difficulty > rand.gen_range(0.0..100.0) as i32 {
        return true;
    }
    return false;
}

/// Return whether the NPC can swing at the moment
pub fn swing_cooldown_check(swing_timer: &mut NPCTimer, time: &Res<Time>) -> bool {
    swing_timer.tick(time.delta());
    if swing_timer.just_finished() {
        return true;
    }
    return false;
}

/// Set the goal to hind behind the closest object to the NPC
pub fn set_tag_to_closest_object(npc_translation: Vec3, path: &mut Path) -> bool {
    let mut rand = thread_rng();
    let recliner = Vec3::new(-60., 210., 1.);
    let tv = Vec3::new(0., -250., 1.);
    let side_table = Vec3::new(120., 170., 1.);
    let recliner_distance = npc_translation.distance(recliner);
    let table_distance = npc_translation.distance(side_table);
    let tv_distance = npc_translation.distance(tv);
    let choice = rand.gen_range(0..4);
    let closest_coords: Vec2;
    if recliner_distance < table_distance {
        if recliner_distance < tv_distance {
            closest_coords = recliner.truncate();
        } else {
            closest_coords = tv.truncate();
        }
    } else {
        if table_distance < tv_distance {
            closest_coords = side_table.truncate();
        } else {
            closest_coords = tv.truncate();
        }
    }
    if choice == 0 {
        path.goal = Vec2::new(closest_coords.x, closest_coords.y + 192.);
    } else if choice == 1 {
        path.goal = Vec2::new(closest_coords.x - 168., closest_coords.y);
    } else if choice == 2 {
        path.goal = Vec2::new(closest_coords.x, closest_coords.y - 192.);
    } else if choice == 3 {
        path.goal = Vec2::new(closest_coords.x + 168., closest_coords.y);
    }

    return true;
}

/// Set NPC's goal to the player
pub fn set_tag_to_player(path: &mut Path, player_translation: Vec3) {
    path.goal = player_translation.truncate().floor();
}

/// Generate a new path using A*
pub fn set_a_star(npc_translation: Vec3, path: &mut Path, maps: &Maps) {
    let goal = path.goal;
    path.set_new_path(a_star(
        coords_conversion_astar(npc_translation.truncate().floor()),
        coords_conversion_astar(goal),
        maps,
    ));
}

/// Movement for Aggression and Evade state, move along the path generated by A*
pub fn perform_a_star(
    mut npcs: Query<
        (&mut Transform, &mut NPCVelocity, &mut Path, &States),
        (With<NPC>, Without<Player>, Without<NPCBat>),
    >,
    mut bat: Query<&mut Transform, (With<NPCBat>, Without<Player>, Without<NPC>)>,
    player: Query<&Transform, (With<Player>, Without<NPCBat>, Without<NPC>)>,
    time: Res<Time>,
) {
    for (mut npc_transform, mut velocity, mut path, state) in npcs.iter_mut() {
        if state.is_aggression() || state.is_evade() {
            for mut bat_transform in bat.iter_mut() {
                for player_transform in player.iter() {
                    let Some(Vec2 { x, y }) = path.path.pop() else {
                        return;
                    };

                    let mut deltav = Vec2::splat(0.);
                    if npc_transform.translation.x < x {
                        deltav.x += 1000.;
                    }
                    if npc_transform.translation.x > x {
                        deltav.x -= 1000.;
                    }
                    if npc_transform.translation.y < y {
                        deltav.y += 1000.;
                    }
                    if npc_transform.translation.y > y {
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

                    if unsafe { MAP == 1 } {
                        velocity.velocity = collision_check_map1(
                            npc_transform.translation,
                            velocity.velocity,
                            player_transform.translation,
                        );
                    } else if unsafe { MAP == 2 || MAP == 3 } {
                        velocity.velocity = collision_check_no_objects(
                            npc_transform.translation,
                            velocity.velocity,
                            player_transform.translation,
                        );
                    } else if unsafe { MAP == 4 } {
                        velocity.velocity = collision_check_map4(
                            npc_transform.translation,
                            velocity.velocity,
                            player_transform.translation,
                        );
                    }

                    velocity.velocity = velocity.velocity * deltat;
                    npc_transform.translation.x = (npc_transform.translation.x
                        + velocity.velocity.x)
                        .clamp(-(1280. / 2.) + NPC_SIZE / 2., 1280. / 2. - NPC_SIZE / 2.);
                    npc_transform.translation.y = (npc_transform.translation.y
                        + velocity.velocity.y)
                        .clamp(-(720. / 2.) + NPC_SIZE / 2., 720. / 2. - NPC_SIZE / 2.);

                    // Fixes Misalign
                    if npc_transform.translation.x != x || npc_transform.translation.y != y {
                        npc_transform.translation.x = x;
                        npc_transform.translation.y = y;
                    }
                    bat_transform.translation.x = npc_transform.translation.x - 5.;
                    bat_transform.translation.y = npc_transform.translation.y;
                } // for
            } // for
        } // if in aggression or evade
    } // for
}

/// Swing and hit the ball
pub fn swing(
    mut npc: Query<
        (
            &mut Transform,
            &mut AnimationTimer,
            &mut NPCTimer,
            &Difficulty,
            &States,
        ),
        (With<NPC>, Without<Ball>, Without<NPCBat>, Without<Player>),
    >,
    mut ball_query: Query<
        (&mut Transform, &mut BallVelocity, &mut Ball),
        (With<Ball>, Without<NPC>, Without<NPCBat>, Without<Player>),
    >,
    player: Query<&Transform, (With<Player>, Without<NPC>, Without<NPCBat>, Without<Ball>)>,
    mut bat: Query<&mut Transform, (With<NPCBat>, Without<NPC>, Without<Ball>, Without<Player>)>,
    time: Res<Time>,
) {
    // Implement swing logic
    for (npc_transform, mut ani_timer, mut swing_timer, difficulty, state) in npc.iter_mut() {
        if state.is_aggression() {
            for mut bat_transform in bat.iter_mut() {
                for player_transform in player.iter() {
                    for (ball_transform, mut ball_velocity, ball) in ball_query.iter_mut() {
                        let ball_x = ball_transform.translation.x;
                        let npc_x = npc_transform.translation.x;
                        let bat_x = bat_transform.translation.x;

                        if ball_transform
                            .translation
                            .distance(npc_transform.translation)
                            < 75.
                            && swing_cooldown_check(&mut swing_timer, &time)
                        {
                            // Check whether the ball is close enough for swinging
                            ball_velocity.velocity = Vec3::splat(0.);
                            let new_velocity = hit_accuracy(
                                npc_transform.translation,
                                player_transform.translation,
                                difficulty.difficulty,
                            );
                            if ball_x > npc_x {
                                bat_transform.scale.x = -0.13;
                            }
                            if ball_x < npc_x {
                                bat_transform.scale.x = 0.13;
                                println!("To the left");
                            }

                            bat_transform.scale.y = -0.13;
                            ball_velocity.velocity = new_velocity * ball.elasticity;
                            //tick animation timer
                            ani_timer.tick(time.delta());
                        }
                    } // for ball_query
                    if ani_timer.just_finished() {
                        bat_transform.scale.y = 0.13;
                    } else {
                        ani_timer.tick(time.delta());
                    }
                } // for player_query
            } // for bat_query
        } // if in aggression
    } // for npc_query
}

/// Determine where to send the ball, deviate the movement vector depending on the difficulty
fn hit_accuracy(npc_translation: Vec3, player_translation: Vec3, difficulty: i32) -> Vec3 {
    let x_diff = player_translation.x - npc_translation.x;
    let y_diff = player_translation.y - npc_translation.y;
    let mut ball_velocity = Vec3::new(x_diff, y_diff, 0.);
    // Implement Hit accuracy deviation based on difficulty
    let mut rand = thread_rng();
    if rand.gen_bool(0.5) {
        if y_diff > x_diff {
            ball_velocity.y = ball_velocity.y - (100 - difficulty) as f32;
        } else {
            ball_velocity.x = ball_velocity.x - (100 - difficulty) as f32;
        }
    } else {
        if y_diff > x_diff {
            ball_velocity.y = ball_velocity.y + (100 - difficulty) as f32;
        } else {
            ball_velocity.x = ball_velocity.x + (100 - difficulty) as f32;
        }
    }

    ball_velocity = ball_velocity.normalize();
    ball_velocity.x = ball_velocity.x * 500.;
    ball_velocity.y = ball_velocity.y * 500.;
    return ball_velocity;
}

/// Check whether collision happened, modify velocity if needed
pub fn collision_check_map1(
    npc_translation: Vec3,
    mut velocity: Vec2,
    player_translation: Vec3,
) -> Vec2 {
    let recliner_size = Vec2::new(109., 184.);
    let recliner_translation = Vec3::new(-60., 210., 1.);
    let recliner = bevy::sprite::collide_aabb::collide(
        recliner_translation,
        recliner_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let tv_size = Vec2::new(164., 103.);
    let tv_translation = Vec3::new(0., -250., 1.);
    let tv_stand = bevy::sprite::collide_aabb::collide(
        tv_translation,
        tv_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let table_size = Vec2::new(103., 107.);
    let table_translation = Vec3::new(120., 170., 1.);
    let side_table = bevy::sprite::collide_aabb::collide(
        table_translation,
        table_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let player_collision = bevy::sprite::collide_aabb::collide(
        player_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    if recliner == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.8;
    } else if recliner == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.8;
    } else if recliner == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.8;
    } else if recliner == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.8;
    }

    if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.9;
    } else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.9;
    } else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.9;
    } else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.9;
    }

    if side_table == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.85;
    } else if side_table == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.85;
    } else if side_table == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.85;
    } else if side_table == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.85;
    }

    if player_collision == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Inside) {
        velocity.x = -1. * 0.85;
        velocity.y = -1. * 0.85;
    }

    return velocity;
}

pub fn collision_check_map4(
    npc_translation: Vec3,
    mut velocity: Vec2,
    player_translation: Vec3,
) -> Vec2 {
    let coral_size = Vec2::new(250., 250.);
    let coral1 = bevy::sprite::collide_aabb::collide(
        Vec3::new(0., 180., 2.),
        coral_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let coral2 = bevy::sprite::collide_aabb::collide(
        Vec3::new(0., 180., 2.),
        coral_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let coral3 = bevy::sprite::collide_aabb::collide(
        Vec3::new(0., 180., 2.),
        coral_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let coral4 = bevy::sprite::collide_aabb::collide(
        Vec3::new(0., 180., 2.),
        coral_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let coral5 = bevy::sprite::collide_aabb::collide(
        Vec3::new(0., 180., 2.),
        coral_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let coral6 = bevy::sprite::collide_aabb::collide(
        Vec3::new(0., 180., 2.),
        coral_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let player_collision = bevy::sprite::collide_aabb::collide(
        player_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    if coral1 == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.9;
    } else if coral1 == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.9;
    } else if coral1 == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.9;
    } else if coral1 == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.9;
    }

    if coral2 == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.9;
    } else if coral2 == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.9;
    } else if coral2 == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.9;
    } else if coral2 == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.9;
    }

    if coral3 == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.9;
    } else if coral3 == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.9;
    } else if coral3 == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.9;
    } else if coral3 == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.9;
    }

    if coral4 == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.9;
    } else if coral4 == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.9;
    } else if coral4 == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.9;
    } else if coral4 == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.9;
    }

    if coral5 == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.9;
    } else if coral5 == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.9;
    } else if coral5 == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.9;
    } else if coral5 == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.9;
    }

    if coral6 == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.9;
    } else if coral6 == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.9;
    } else if coral6 == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.9;
    } else if coral6 == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.9;
    }

    if player_collision == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Inside) {
        velocity.x = -1. * 0.85;
        velocity.y = -1. * 0.85;
    }
    return velocity;
}

pub fn collision_check_no_objects(
    npc_translation: Vec3,
    mut velocity: Vec2,
    player_translation: Vec3,
) -> Vec2 {
    let player_collision = bevy::sprite::collide_aabb::collide(
        player_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    if player_collision == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Inside) {
        velocity.x = -1. * 0.85;
        velocity.y = -1. * 0.85;
    }

    return velocity;
}
