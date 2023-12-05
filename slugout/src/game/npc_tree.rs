use std::sync::Arc;
use std::sync::Mutex;

use crate::game::npc::*;
use crate::game::npc_events::*;
use crate::game::pathfinding::*;
use crate::GameState;
use bevy::prelude::*;
use bevy::utils::synccell::SyncCell;
//use bevy::time::Stopwatch;

use super::components::Ball;
use super::components::Object;
use super::components::Player;
use super::npc;

/*  In rememberance of the Box version tree
 *  Ran into synchronisation issues with bevy's concurrency and variable lifetime so didn't workout
 *  Maybe Someday, Welp we Tried D:
 */

pub enum Node {
    Sequence(Vec<Node>),
    Fallback(Vec<Node>),
    Condition(
        Box<
            dyn Fn(
                Query<
                    // NPC Query
                    (
                        &mut Transform,
                        &mut NPCVelocity,
                        &mut Path,
                        &Maps,
                        &Difficulty,
                        &mut AnimationTimer,
                    ),
                    With<NPC>,
                >,
                Query<&Transform, With<Ball>>,
                Query<&Transform, With<Player>>,
                Query<&Transform, With<Object>>,
            ) -> bool,
        >,
    ),
    Action(
        Box<
            dyn Fn(
                Query<
                    (
                        &mut Transform,
                        &mut NPCVelocity,
                        &mut Path,
                        &Maps,
                        &Difficulty,
                        &mut AnimationTimer,
                    ),
                    With<NPC>,
                >,
                Query<&Transform, With<Ball>>,
                Query<&Transform, With<Player>>,
                Query<&Transform, With<Object>>,
            ) -> NodeStatus,
        >,
    ),
    Decorator(
        Box<
            dyn Fn(
                Query<
                    (
                        &mut Transform,
                        &mut NPCVelocity,
                        &mut Path,
                        &Maps,
                        &Difficulty,
                        &mut AnimationTimer,
                    ),
                    With<NPC>,
                >,
                Query<&Transform, With<Ball>>,
                Query<&Transform, With<Player>>,
                Query<&Transform, With<Object>>,
            ) -> bool,
        >,
        Box<Node>,
    ),
}

#[derive(Component, PartialEq, Eq, Debug, Clone)]
enum NodeStatus {
    Success,
    Failure,
    Running,
}

// unsafe impl Send for Node {}
/*
 */

// fn behaviour_tree_setup(mut commands: Commands) {
//     let npc_entity = commands.spawn(NPC).id();
//     let sequence_node = commands.spawn(Sequence).id();
//     let fallback_node = commands.spawn(Fallback).id();
//     let decorator_node = commands.spawn(Decorator).id();
//     let condition_node = commands.spawn(Decorator).id();

//     commands
//         .entity(npc_entity)
//         .insert(NPC)
//         .insert(Fallback)
//         .with_children(|npc_entity| {
//             npc_entity
//                 .spawn(sequence_node)
//                 .insert(Sequence)
//                 .with_children(|sequence| {
//                     sequence.spawn().insert(MoveToDonut { agent });
//                     sequence.spawn().insert(EatDonut { agent });
//                 });
//         });
// }

pub fn behavior_tree(
    mut npc: Query<
        (
            &mut Transform,
            &mut NPCVelocity,
            &mut Path,
            &Maps,
            &Difficulty,
            &mut AnimationTimer,
        ),
        With<NPC>,
    >,
    ball_query: &Query<&Transform, With<Ball>>,
    player_query: &Query<&Transform, With<Player>>,
    object_query: &Query<&Transform, With<Object>>,
) {
    let root_node = Node::Fallback(vec![
        Node::Sequence(vec![
            Node::Condition(Box::new(|npc, ball_query, player_query, object_query| {
                danger_check(npc, ball_query)
            })),
            Node::Fallback(vec![
                Node::Condition(Box::new(|npc, _, _, _| aggression_check(npc))),
                Node::Condition(Box::new(|npc, _, _, _| swing_cooldown_check(npc))),
                Node::Sequence(vec![
                    Node::Action(Box::new(|npc, ball_query, player_query, object_query| {
                        swing(npc, ball_query)
                    })),
                    // Add more nodes for SWING subtree
                ]),
            ]),
            Node::Action(Box::new(|npc, _, _, _| sidestep(npc))),
        ]),
        Node::Sequence(vec![
            Node::Decorator(
                Box::new(|npc, _, _, _| player_proximity_check(npc, *player_query)),
                Box::new(Node::Fallback(vec![Node::Sequence(vec![
                    Node::Condition(Box::new(|npc, _, _, _| aggression_check(npc))),
                    Node::Action(Box::new(|npc, ball_query, player_query, object_query| {
                        swing(npc, ball_query)
                    })),
                    // Add more nodes for SWING subtree
                ])])),
            ),
            Node::Decorator(
                Box::new(|npc, _, _, _| tag_is_null(npc)),
                Box::new(Node::Fallback(vec![
                    Node::Sequence(vec![
                        Node::Condition(Box::new(|npc, _, _, _| aggression_check(npc))),
                        Node::Action(Box::new(|npc, ball_query, player_query, object_query| {
                            set_tag_to_closest_ball(npc, ball_query)
                        })),
                    ]),
                    Node::Action(Box::new(|npc, _, _, _| {
                        set_tag_to_closest_object(npc, *object_query)
                    })),
                ])),
            ),
            Node::Action(Box::new(|npc, _, _, _| perform_a_star(npc))),
        ]),
    ]);
}

pub fn execute_node(
    node: Node,
    mut npc: Query<
        (
            &mut Transform,
            &mut NPCVelocity,
            &mut Path,
            &Maps,
            &Difficulty,
            &mut AnimationTimer,
        ),
        With<NPC>,
    >,
    commands: &mut Commands,
    ball_query: Query<&Transform, With<Ball>>,
    player_query: Query<&Transform, With<Player>>,
    object_query: Query<&Transform, With<Object>>,
) -> NodeStatus {
    match node {
        Node::Sequence(children) => {
            for child in children {
                let status =
                    execute_node(child, npc, commands, ball_query, player_query, object_query);
                match status {
                    // If status != success, return current status
                    NodeStatus::Success => {
                        continue;
                    }
                    NodeStatus::Failure => {
                        return status;
                    }
                    NodeStatus::Running => {
                        return status;
                    }
                }
            }
            return NodeStatus::Success;
        }
        Node::Fallback(children) => {
            for child in children {
                let status =
                    execute_node(child, npc, commands, ball_query, player_query, object_query);
                match status {
                    // If status != failure, return current status
                    NodeStatus::Success => {
                        return status;
                    }
                    NodeStatus::Failure => {
                        continue;
                    }
                    NodeStatus::Running => {
                        return status;
                    }
                }
            }
            return NodeStatus::Failure;
        }
        Node::Condition(condition) => {
            if condition(npc, ball_query, player_query, object_query) {
                return NodeStatus::Success;
            } else {
                return NodeStatus::Failure;
            }
        }
        Node::Action(action) => action(npc, ball_query, player_query, object_query),
        Node::Decorator(decorator_condition, child) => {
            if decorator_condition(npc, ball_query, player_query, object_query) {
                execute_node(
                    *child,
                    npc,
                    commands,
                    ball_query,
                    player_query,
                    object_query,
                )
            } else {
                NodeStatus::Failure
            }
        }
    }
}

fn danger_check(
    mut npcs: Query<
        (
            &mut Transform,
            &mut NPCVelocity,
            &mut Path,
            &Maps,
            &Difficulty,
            &mut AnimationTimer,
        ),
        With<NPC>,
    >,
    ball_query: Query<&Transform, With<Ball>>,
    // _: &QuerySet<(Entity, &Player)>,
    // _: &QuerySet<(Entity, &GameObject)>,
) -> bool {
    true
}

fn sidestep(
    mut npcs: Query<
        (
            &mut Transform,
            &mut NPCVelocity,
            &mut Path,
            &Maps,
            &Difficulty,
            &mut AnimationTimer,
        ),
        With<NPC>,
    >,
) -> NodeStatus {
    // Implement sidestep logic
    return NodeStatus::Success;
}

fn player_proximity_check(
    mut npcs: Query<
        (
            &mut Transform,
            &mut NPCVelocity,
            &mut Path,
            &Maps,
            &Difficulty,
            &mut AnimationTimer,
        ),
        With<NPC>,
    >,
    player_query: Query<&Transform, With<Player>>,
) -> bool {
    // Implement player proximity check logic
    return true;
}

fn tag_is_null(
    mut npcs: Query<
        (
            &mut Transform,
            &mut NPCVelocity,
            &mut Path,
            &Maps,
            &Difficulty,
            &mut AnimationTimer,
        ),
        With<NPC>,
    >,
) -> bool {
    // Implement TAG null check logic
    return true;
}

fn set_tag_to_closest_ball(
    mut npcs: Query<
        (
            &mut Transform,
            &mut NPCVelocity,
            &mut Path,
            &Maps,
            &Difficulty,
            &mut AnimationTimer,
        ),
        With<NPC>,
    >,
    ball_query: Query<&Transform, With<Ball>>,
) -> NodeStatus {
    // Implement setting TAG to the closest ball logic
    return NodeStatus::Success;
}
fn aggression_check(
    mut npcs: Query<
        (
            &mut Transform,
            &mut NPCVelocity,
            &mut Path,
            &Maps,
            &Difficulty,
            &mut AnimationTimer,
        ),
        With<NPC>,
    >,
) -> bool {
    return true;
}

fn swing_cooldown_check(
    mut npcs: Query<
        (
            &mut Transform,
            &mut NPCVelocity,
            &mut Path,
            &Maps,
            &Difficulty,
            &mut AnimationTimer,
        ),
        With<NPC>,
    >,
) -> bool {
    return true;
}
fn set_tag_to_closest_object(
    mut npcs: Query<
        (
            &mut Transform,
            &mut NPCVelocity,
            &mut Path,
            &Maps,
            &Difficulty,
            &mut AnimationTimer,
        ),
        With<NPC>,
    >,
    object_query: Query<&Transform, With<Object>>,
) -> NodeStatus {
    // Implement setting TAG to the closest object logic
    return NodeStatus::Success;
}

fn perform_a_star(
    mut npcs: Query<
        (
            &mut Transform,
            &mut NPCVelocity,
            &mut Path,
            &Maps,
            &Difficulty,
            &mut AnimationTimer,
        ),
        With<NPC>,
    >,
) -> NodeStatus {
    // Implement A* pathfinding logic
    for (npc_transform, _velocity, mut path, maps, _diffculty, _animation_timer) in npcs.iter_mut()
    {
        let goal = path.goal;
        path.set_new_path(a_star(
            coords_conversion_astar(npc_transform.translation.truncate().floor()),
            coords_conversion_astar(goal),
            maps,
        ));
    }
    return NodeStatus::Success;
}

fn swing(
    mut npcs: Query<
        (
            &mut Transform,
            &mut NPCVelocity,
            &mut Path,
            &Maps,
            &Difficulty,
            &mut AnimationTimer,
        ),
        With<NPC>,
    >,
    ball_query: Query<&Transform, With<Ball>>,
    // _: &QuerySet<(Entity, &Player)>,
    // _: &QuerySet<(Entity, &GameObject)>,
) -> NodeStatus {
    // Implement swing logic
    return NodeStatus::Success;
}
