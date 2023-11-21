use std::time::Duration;

use super::components::Player;
use crate::game::npc::*;
use crate::game::pathfinding::*;
use crate::GameState;
use bevy::prelude::*;
use bevy::time::common_conditions::*;

const NPC_SIZE: f32 = 30.;
// 5px/frame @60Hz == 300px/s
const NPC_SPEED: f32 = 300.;
// 1px/frame^2 @60Hz == 3600px/s^2
const NPC_ACCEL_RATE: f32 = 18000.;
pub struct NPCBullyPlugin;

impl Plugin for NPCBullyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), load_npc);
        //app.add_systems(OnEnter(GameState::Game), load_map);
        app.add_systems(
            Update,
            set_path
                .run_if(in_state(GameState::Game))
                .run_if(on_timer(Duration::from_secs(1))),
        );
        app.add_systems(
            Update,
            approach_player_bully.run_if(in_state(GameState::Game)), //  .run_if(on_timer(Duration::from_secs(1))),
        );
        //  app.add_systems(Update, bat_swing.run_if(in_state(GameState::Game)));
    }
}
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

pub fn approach_player_bully(
    mut npcs: Query<
        (&mut Transform, &mut Path),
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
    //time: Res<Time>,
) {
    for (mut npc_transform, mut path) in npcs.iter_mut() {
        for mut bat_transform in bat.iter_mut() {
            for mut face_transform in face.iter_mut() {
                let Some(Vec2 { x, y }) = path.path.pop() else {
                    return;
                };
                npc_transform.translation.x = x;
                npc_transform.translation.y = y;
                bat_transform.translation.x = npc_transform.translation.x - 5.;
                bat_transform.translation.y = npc_transform.translation.y;
                face_transform.translation.x = npc_transform.translation.x;
                face_transform.translation.y = npc_transform.translation.y;
            }
        }
    }
}
