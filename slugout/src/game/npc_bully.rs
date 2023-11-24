use std::time::Duration;

use super::components::Player;
use crate::game::npc::*;
use crate::game::pathfinding::*;
use crate::GameState;
use bevy::prelude::*;
use bevy::time::common_conditions::*;

const TIMESTEP_1: f64 = 60.0 / 60.0;

const NPC_SIZE: f32 = 30.;
// 5px/frame @60Hz == 300px/s
const NPC_SPEED: f32 = 300.;
// 1px/frame^2 @60Hz == 3600px/s^2
const NPC_ACCEL_RATE: f32 = 18000.;
pub struct NPCBullyPlugin;

impl Plugin for NPCBullyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), load_npc);
        app.add_systems(OnEnter(GameState::Game), load_map);
        app.add_systems(
            Update,
            approach_player_bully
                .run_if(in_state(GameState::Game))
                .run_if(on_timer(Duration::from_secs(1))),
        );
        //  app.add_systems(Update, bat_swing.run_if(in_state(GameState::Game)));
    }
}

pub fn approach_player_bully(
    mut npcs: Query<
        (&mut Transform, &mut NPCVelocity, &Maps),
        (
            With<NPC>,
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
            Without<NPCFace>,
        ),
    >,
    mut face: Query<
        &mut Transform,
        (
            With<NPCFace>,
            Without<Player>,
            Without<NPC>,
            Without<NPCBat>,
        ),
    >,
    player: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    for (mut npc_transform, mut velocity, maps) in npcs.iter_mut() {
        for player_transform in player.iter() {
            for mut bat_transform in bat.iter_mut() {
                for mut face_transform in face.iter_mut() {
                    let route_vec = a_star(
                        coords_conversion_astar(npc_transform.translation.truncate().floor()),
                        coords_conversion_astar(player_transform.translation.truncate().floor()),
                        maps,
                    );
                    let mut deltav = Vec2::splat(0.);
                    for dest in route_vec {
                        if npc_transform.translation.x < dest.x {
                            deltav.x += 1000.;
                        }
                        if npc_transform.translation.x > dest.x {
                            deltav.x -= 1000.;
                        }
                        if npc_transform.translation.y < dest.y {
                            deltav.y += 1000.;
                        }
                        if npc_transform.translation.y > dest.y {
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
                        deltav = Vec2::splat(0.);
                    }
                }
            }
        }
    }
}
