use crate::game::npc::*;
use crate::game::npc_events::*;
use crate::game::pathfinding::*;
use crate::GameState;
use bevy::prelude::*;

use super::components::Ball;
use super::components::Player;

pub struct NPCBullyPlugin;

impl Plugin for NPCBullyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), load_npc);
        app.add_systems(OnEnter(GameState::Game), load_map);
        app.add_systems(Update, avoid_collision.run_if(in_state(GameState::Game)));
        app.add_systems(
            Update,
            approach_player
                .after(select)
                .run_if(in_state(GameState::Game)),
        );
        app.add_systems(
            Update,
            bat_swing.after(select).run_if(in_state(GameState::Game)),
        );
    }
}

pub fn approach_player_bully(
    mut npcs: Query<
        (&mut Transform, &mut NPCVelocity),
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
) {
    //  let x = a_star();
}
