use std::time::Duration;

use super::components::*;
use crate::game::npc::*;
use crate::game::npc_events::*;
use crate::game::pathfinding::*;
use crate::GameState;
use bevy::prelude::*;
use bevy::time::common_conditions::*;

pub struct NPCBullyPlugin;

impl Plugin for NPCBullyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), load_npc);
        //app.add_systems(OnEnter(GameState::Game), load_map);
        app.add_systems(
            Update,
            set_path
                .run_if(in_state(GameState::Game))
                .run_if(on_timer(Duration::from_secs(10))),
        );
        app.add_systems(
            Update,
            approach_player.run_if(in_state(GameState::Game)), //.run_if(on_fixed_timer(Duration::from_millis(50))), //  .run_if(on_timer(Duration::from_secs(1))),
        );
        app.add_systems(Update, swing.run_if(in_state(GameState::Game)));
        //  app.add_systems(Update, bat_swing.run_if(in_state(GameState::Game)));
    }
}
pub fn set_path(
    mut npcs: Query<(&Transform, &Maps, &mut Path), (With<NPC>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<NPC>)>,
) {
    for (npc_transform, maps, mut path) in npcs.iter_mut() {
        //  thread::spawn(move || {
        for player_transform in player.iter() {
            //thread::spawn(move || {
            path.set_new_path(a_star(
                coords_conversion_astar(npc_transform.translation.truncate().floor()),
                coords_conversion_astar(player_transform.translation.truncate().floor()),
                maps,
            ));
            //    });
        }

        //  });
    }
}

// pub fn approach_player_bully(
//     mut npcs: Query<
//         (&mut Transform, &mut Path, &mut NPCVelocity),
//         (
//             With<NPC>,
//             Without<Player>,
//             Without<NPCBat>,
//             Without<NPCFace>,
//         ),
//     >,
//     mut bat: Query<
//         &mut Transform,
//         (
//             With<NPCBat>,
//             Without<Player>,
//             Without<NPC>,
//             Without<NPCFace>,
//         ),
//     >,
//     mut face: Query<
//         &mut Transform,
//         (
//             With<NPCFace>,
//             Without<Player>,
//             Without<NPC>,
//             Without<NPCBat>,
//         ),
//     >,
//     player: Query<&Transform, (With<Player>, Without<NPC>)>,
//     time: Res<Time>,
// ) {
//     for (mut npc_transform, mut path, mut _velocity) in npcs.iter_mut() {
//         for mut bat_transform in bat.iter_mut() {
//             for mut face_transform in face.iter_mut() {
//                 let Some(Vec2 { x, y }) = path.path.pop() else {
//                     return;
//                 };
//                 if npc_transform.translation.x < x {
//                     npc_transform.translation.x += 2.;
//                 }
//                 if npc_transform.translation.x > x {
//                     npc_transform.translation.x -= 2.;
//                 }
//                 if npc_transform.translation.y < y {
//                     npc_transform.translation.y += 2.;
//                 }
//                 if npc_transform.translation.y > y {
//                     npc_transform.translation.y -= 2.;
//                 }
//                 // npc_transform.translation.x = x;
//                 // npc_transform.translation.y = y;
//                 bat_transform.translation.x = npc_transform.translation.x - 5.;
//                 bat_transform.translation.y = npc_transform.translation.y;
//                 face_transform.translation.x = npc_transform.translation.x;
//                 face_transform.translation.y = npc_transform.translation.y;
//             }
//         }
//     }
// }

pub fn swing(
    // mut ball: Query<
    //     (&mut Ball, &mut BallVelocity, &mut Transform),
    //     (With<Ball>, Without<Hitbox>, Without<Bat>, Without<Player>),
    // >,
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
        if Vec2::distance(
            npc_transform.translation.truncate(),
            player_transform.translation.truncate(),
        )
        .abs()
            < 30.
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
