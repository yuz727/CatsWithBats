/// Imports
use super::components::*;

use crate::game::npc::*;
use crate::game::npc_events::collision_check;
use crate::game::pathfinding::*;

use bevy::prelude::*;
// use bevy::time::common_conditions::*;
// use std::time::Duration;

/// Constants for NPC movement
const NPC_SIZE: f32 = 30.;
const NPC_SPEED: f32 = 300.;
const NPC_ACCEL_RATE: f32 = 18000.;

// /// Overall Logic for the easter egg ver.
// /// Go to player, if close enough, smack them with the bat repeatedly
// /// Have Fun :D

/// Set path to player using A* algorithm
pub fn set_path(
    mut npcs: Query<(&Transform, &Maps, &mut Path), (With<NPC>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<NPC>)>,
) {
    for (npc_transform, maps, mut path) in npcs.iter_mut() {
        for player_transform in player.iter() {
            path.set_new_path(a_star(
                coords_conversion_astar(npc_transform.translation.truncate().floor()),
                coords_conversion_astar(player_transform.translation.truncate().floor()),
                maps,
            ));
        }
    }
}

///  Move towards the player using path generated by the A* algorithm
pub fn approach_player(
    mut npcs: Query<
        (&mut Transform, &mut NPCVelocity, &mut Path, &Maps),
        (With<NPC>, Without<Ball>, Without<Player>, Without<NPCBat>),
    >,
    mut bat: Query<&mut Transform, (With<NPCBat>, Without<Player>, Without<NPC>, Without<Ball>)>,
    player: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    for (mut npc_transform, mut velocity, mut path, maps) in npcs.iter_mut() {
        for mut bat_transform in bat.iter_mut() {
            for player_transform in player.iter() {
                //  for mut face_transform in face.iter_mut() {
                let Some(Vec2 { x, y }) = path.path.pop() else {
                    // If there's no more points to move generate a new path
                    path.set_new_path(a_star(
                        coords_conversion_astar(npc_transform.translation.truncate().floor()),
                        coords_conversion_astar(player_transform.translation.truncate().floor()),
                        maps,
                    ));
                    return;
                };

                // Movement calculation
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

                let old_vel = velocity.velocity;
                let mut collision: bool = false;
                // Modify velocity based on whether collision happened
                velocity.velocity = collision_check(
                    npc_transform.translation,
                    velocity.velocity,
                    player_transform.translation,
                );

                if old_vel.x != velocity.velocity.x || old_vel.y != velocity.velocity.y {
                    collision = true;
                }

                velocity.velocity = velocity.velocity * deltat;
                npc_transform.translation.x = (npc_transform.translation.x + velocity.velocity.x)
                    .clamp(-(1280. / 2.) + NPC_SIZE / 2., 1280. / 2. - NPC_SIZE / 2.);
                npc_transform.translation.y = (npc_transform.translation.y + velocity.velocity.y)
                    .clamp(-(720. / 2.) + NPC_SIZE / 2., 720. / 2. - NPC_SIZE / 2.);

                // Fixes Misalign caused by the pathfinding grids being 4x4 pixel chunks
                if (npc_transform.translation.x != x || npc_transform.translation.y != y)
                    && !collision
                {
                    npc_transform.translation.x = x;
                    npc_transform.translation.y = y;
                }
                bat_transform.translation.x = npc_transform.translation.x - 5.;
                bat_transform.translation.y = npc_transform.translation.y;
            }
        }
    }
}

/// Bonk.
pub fn bully_swing(
    mut bat: Query<
        &mut Transform,
        (
            With<NPCBat>,
            Without<Hitbox>,
            Without<Ball>,
            Without<Player>,
            Without<NPC>,
        ),
    >,
    mut npcs: Query<
        (&mut Transform, &mut AnimationTimer, &mut NPCTimer),
        (With<NPC>, Without<Player>),
    >,
    player: Query<
        &Transform,
        (
            With<Player>,
            Without<Hitbox>,
            Without<NPCBat>,
            Without<Ball>,
        ),
    >,
    time: Res<Time>,
) {
    for (npc_transform, mut ani_timer, mut swing_timer) in npcs.iter_mut() {
        let mut bat_transform = bat.single_mut();
        let player_transform = player.single();

        // Swing if the NPC is close enough ot the player & the swing interval has passed
        if Vec2::distance(
            npc_transform.translation.truncate(),
            player_transform.translation.truncate(),
        )
        .abs()
            < 50.
        {
            swing_timer.tick(time.delta());
            ani_timer.tick(time.delta());
            if swing_timer.just_finished() {
                if ani_timer.just_finished() {
                    bat_transform.scale.y = -0.13;
                } else {
                    bat_transform.scale.y = 0.13;
                }
            }
        }
    }
}
