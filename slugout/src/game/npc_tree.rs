use crate::game::npc::*;
use crate::game::npc_events::*;
use crate::game::pathfinding::*;
use crate::GameState;
use bevy::prelude::*;
//use bevy::time::Stopwatch;

use super::components::Ball;
use super::components::Object;
use super::components::Player;

enum Node {
    Sequence(Vec<Node>),
    Fallback(Vec<Node>),
    Condition(
        Box<
            dyn Fn(
                Query<&Transform, With<NPC>>,
                Query<&Transform, With<Ball>>,
                Query<&Transform, With<Player>>,
                Query<&Transform, With<Object>>,
            ) -> bool,
        >,
    ),
    Action(
        Box<
            dyn Fn(
                Query<&Transform, With<NPC>>,
                &mut Commands,
                Query<&Transform, With<Ball>>,
                Query<&Transform, With<Player>>,
                Query<&Transform, With<Object>>,
            ) -> NodeStatus,
        >,
    ),
    Decorator(
        Box<
            dyn Fn(
                Query<&Transform, With<NPC>>,
                Query<&Transform, With<Ball>>,
                Query<&Transform, With<Player>>,
                Query<&Transform, With<Object>>,
            ) -> bool,
        >,
        Box<Node>,
    ),
}

enum NodeStatus {
    Success,
    Failure,
    Running,
}

fn behavior_tree(
    npc: Query<(&mut Transform, &mut NPCVelocity, &mut States), With<NPC>>,
    commands: &mut Commands,
    ball_query: Query<&Transform, With<Ball>>,
    player_query: Query<&Transform, With<Player>>,
    object_query: Query<&Transform, With<Object>>,
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
                    Node::Action(Box::new(
                        |npc, commands, ball_query, player_query, object_query| {
                            swing(&mut npc, commands, ball_query)
                        },
                    )),
                    // Add more nodes for SWING subtree
                ]),
            ]),
            Node::Action(Box::new(|npc, commands, _, _, _| sidestep(npc, commands))),
        ]),
        Node::Sequence(vec![
            Node::Decorator(
                Box::new(|npc, _, _, _| player_proximity_check(npc)),
                Box::new(Node::Fallback(vec![Node::Sequence(vec![
                    Node::Condition(Box::new(|npc, _, _, _| aggression_check(npc))),
                    Node::Action(Box::new(
                        |npc, commands, ball_query, player_query, object_query| {
                            swing(npc, commands, ball_query)
                        },
                    )),
                    // Add more nodes for SWING subtree
                ])])),
            ),
            Node::Decorator(
                Box::new(|npc, _, _, _| tag_is_null(npc)),
                Box::new(Node::Fallback(vec![
                    Node::Sequence(vec![
                        Node::Condition(Box::new(|npc, _, _, _| aggression_check(npc))),
                        Node::Action(Box::new(
                            |npc, commands, ball_query, player_query, object_query| {
                                set_tag_to_closest_ball(npc, commands, ball_query)
                            },
                        )),
                    ]),
                    Node::Action(Box::new(|npc, commands, _, _, _| {
                        set_tag_to_closest_object(npc, object_query)
                    })),
                ])),
            ),
            Node::Action(Box::new(|npc, commands, _, _, _| perform_a_star(npc))),
        ]),
    ]);

    execute_node(
        root_node,
        npc,
        commands,
        ball_query,
        player_query,
        object_query,
    );
}

fn execute_node(
    node: Node,
    npc: Query<&Transform, With<NPC>>,
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
        Node::Action(action) => action(npc, commands, ball_query, player_query, object_query),
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
    npc: &NPC,
    ball_query: Query<&Transform, With<Ball>>,
    // _: &QuerySet<(Entity, &Player)>,
    // _: &QuerySet<(Entity, &GameObject)>,
) -> bool {
    true
}

fn aggression_check(npc: &NPC) -> bool {
    return true;
}

fn swing_cooldown_check(npc: &NPC) -> bool {
    return true;
}

fn sidestep(
    npc: &mut NPC,
    commands: &mut Commands,
    // _: &QuerySet<(Entity, &Ball)>,
    // _: &QuerySet<(Entity, &Player)>,
    // _: &QuerySet<(Entity, &GameObject)>,
) -> NodeStatus {
    // Implement sidestep logic
    return NodeStatus::Success;
}

fn player_proximity_check(
    npc: &NPC,
    //  _: &QuerySet<(Entity, &Ball)>,
    player_query: Query<&Transform, With<Player>>,
    //  _: &QuerySet<(Entity, &GameObject)>,
) -> bool {
    // Implement player proximity check logic
    return true;
}

fn tag_is_null(
    npc: &NPC,
    // _: &QuerySet<(Entity, &Ball)>,
    // _: &QuerySet<(Entity, &Player)>,
    // _: &QuerySet<(Entity, &GameObject)>,
) -> bool {
    // Implement TAG null check logic
    return true;
}

fn set_tag_to_closest_ball(
    npc: &mut NPC,
    commands: &mut Commands,
    ball_query: Query<&Transform, With<Ball>>,
    // _: &QuerySet<(Entity, &Player)>,
    // _: &QuerySet<(Entity, &GameObject)>,
) -> NodeStatus {
    // Implement setting TAG to the closest ball logic
    return NodeStatus::Success;
}

fn set_tag_to_closest_object(
    npc: &mut NPC,
    //  commands: &mut Commands,
    // _: &QuerySet<(Entity, &Ball)>,
    // _: &QuerySet<(Entity, &Player)>,
    object_query: Query<&Transform, With<Object>>,
) -> NodeStatus {
    // Implement setting TAG to the closest object logic
    return NodeStatus::Success;
}

fn perform_a_star(
    mut npcs: Query<(&Transform, &Maps, &mut Path), (With<NPC>, Without<Player>)>,
    // _: &QuerySet<(Entity, &Ball)>,
    // _: &QuerySet<(Entity, &Player)>,
    // _: &QuerySet<(Entity, &GameObject)>,
) -> NodeStatus {
    // Implement A* pathfinding logic
    for (npc_transform, maps, mut path) in npcs.iter_mut() {
        path.set_new_path(a_star(
            coords_conversion_astar(npc_transform.translation.truncate().floor()),
            coords_conversion_astar(path.goal),
            maps,
        ));
    }
    return NodeStatus::Success;
}

fn swing(
    npc: &mut NPC,
    commands: &mut Commands,
    ball_query: Query<&Transform, With<Ball>>,
    // _: &QuerySet<(Entity, &Player)>,
    // _: &QuerySet<(Entity, &GameObject)>,
) -> NodeStatus {
    // Implement swing logic
    return NodeStatus::Success;
}
