use bevy::prelude::*;

// Components
struct Npc {
    difficulty: u32, // Adjust the type according to your game's specifics
    last_swing_time: f64, // Timestamp of the last swing
}

struct Ball;
struct Player;
struct GameObject;

// Behavior Tree Nodes
enum NodeStatus {
    Success,
    Failure,
    Running,
}

type NodeFunction = fn(&mut Npc, &mut Commands, &QuerySet<(Entity, &Ball)>, &QuerySet<(Entity, &Player)>, &QuerySet<(Entity, &GameObject)>) -> NodeStatus;

struct ConditionNode {
    condition: NodeFunction,
}

struct ActionNode {
    action: NodeFunction,
}

struct SequenceNode {
    children: Vec<Box<dyn TreeNode>>,
}

struct FallbackNode {
    children: Vec<Box<dyn TreeNode>>,
}

// Trait for all behavior tree nodes
trait TreeNode {
    fn execute(&mut self, npc: &mut Npc, commands: &mut Commands, ball_query: &QuerySet<(Entity, &Ball)>, player_query: &QuerySet<(Entity, &Player)>, object_query: &QuerySet<(Entity, &GameObject)>) -> NodeStatus;
}

impl TreeNode for ConditionNode {
    fn execute(&mut self, npc: &mut Npc, _: &mut Commands, ball_query: &QuerySet<(Entity, &Ball)>, player_query: &QuerySet<(Entity, &Player)>, object_query: &QuerySet<(Entity, &GameObject)>) -> NodeStatus {
        (self.condition)(npc, &mut Commands::default(), ball_query, player_query, object_query)
    }
}

impl TreeNode for ActionNode {
    fn execute(&mut self, npc: &mut Npc, commands: &mut Commands, ball_query: &QuerySet<(Entity, &Ball)>, player_query: &QuerySet<(Entity, &Player)>, object_query: &QuerySet<(Entity, &GameObject)>) -> NodeStatus {
        (self.action)(npc, commands, ball_query, player_query, object_query)
    }
}

impl TreeNode for SequenceNode {
    fn execute(&mut self, npc: &mut Npc, commands: &mut Commands, ball_query: &QuerySet<(Entity, &Ball)>, player_query: &QuerySet<(Entity, &Player)>, object_query: &QuerySet<(Entity, &GameObject)>) -> NodeStatus {
        for child in &mut self.children {
            let status = child.execute(npc, commands, ball_query, player_query, object_query);
            if status != NodeStatus::Success {
                return status;
            }
        }
        NodeStatus::Success
    }
}

impl TreeNode for FallbackNode {
    fn execute(&mut self, npc: &mut Npc, commands: &mut Commands, ball_query: &QuerySet<(Entity, &Ball)>, player_query: &QuerySet<(Entity, &Player)>, object_query: &QuerySet<(Entity, &GameObject)>) -> NodeStatus {
        for child in &mut self.children {
            let status = child.execute(npc, commands, ball_query, player_query, object_query);
            if status != NodeStatus::Failure {
                return status;
            }
        }
        NodeStatus::Failure
    }
}

// Behavior Tree Setup
fn setup(mut commands: Commands) {
    commands.spawn().insert(Npc {
        difficulty: 50,
        last_swing_time: 0.0,
    });

    // Spawn entities for the game world (balls, players, objects)
    // Add Ball, Player, and GameObject components to respective entities
}

// Behavior Tree Execution
fn behavior_tree(npc: &mut Npc, commands: &mut Commands, ball_query: &QuerySet<(Entity, &Ball)>, player_query: &QuerySet<(Entity, &Player)>, object_query: &QuerySet<(Entity, &GameObject)>) {
    let root_node = FallbackNode {
        children: vec![
            Box::new(SequenceNode {
                children: vec![
                    Box::new(ConditionNode { condition: danger_check }),
                    Box::new(FallbackNode {
                        children: vec![
                            Box::new(ConditionNode { condition: aggression_check }),
                            Box::new(ConditionNode { condition: swing_cooldown_check }),
                            Box::new(SequenceNode {
                                children: vec![
                                    Box::new(ActionNode { action: swing }),
                                    // Add more nodes for SWING subtree
                                ],
                            }),
                        ],
                    }),
                    Box::new(ActionNode { action: sidestep }),
                ],
            }),
            Box::new(SequenceNode {
                children: vec![
                    Box::new(ActionNode { action: player_proximity_check }),
                    Box::new(ConditionNode { condition: aggression_check }),
                    Box::new(ConditionNode { condition: swing_cooldown_check }),
                    Box::new(SequenceNode {
                        children: vec![
                            Box::new(ActionNode { action: swing }),
                            // Add more nodes for SWING subtree
                        ],
                    }),
                ],
            }),
            Box::new(FallbackNode {
                children: vec![
                    Box::new(SequenceNode {
                        children: vec![
                            Box::new(ConditionNode { condition: aggression_check }),
                            Box::new(ActionNode { action: set_tag_to_closest_ball }),
                        ],
                    }),
                    Box::new(ActionNode { action: set_tag_to_closest_object }),
                ],
            }),
            Box::new(ActionNode { action: perform_a_star }),
        ],
    };

    // Execute the behavior tree
    root_node.execute(npc, commands, ball_query, player_query, object_query);
}

// Define your condition and action functions here

fn danger_check(npc: &mut Npc, _: &mut Commands, ball_query: &QuerySet<(Entity, &Ball)>, _: &QuerySet<(Entity, &Player)>, _: &QuerySet<(Entity, &GameObject)>) -> NodeStatus {
    // Implement danger check logic
    NodeStatus::Success
}

fn aggression_check(npc: &mut Npc, _: &mut Commands, _: &QuerySet<(Entity, &Ball)>, _: &QuerySet<(Entity, &Player)>, _: &QuerySet<(Entity, &GameObject)>) -> NodeStatus {
    // Implement aggression check logic
    NodeStatus::Success
}

fn swing_cooldown_check(npc: &mut Npc, _: &mut Commands, _: &QuerySet<(Entity, &Ball)>, _: &QuerySet<(Entity, &Player)>, _: &QuerySet<(Entity, &GameObject)>) -> NodeStatus {
    // Implement swing cooldown check logic
    NodeStatus::Success
}

fn sidestep(npc: &mut Npc, _: &mut Commands, _: &QuerySet<(Entity, &Ball)>, _: &QuerySet<(Entity, &Player)>, _: &QuerySet<(Entity, &GameObject)>) -> NodeStatus {
    // Implement sidestep logic
    NodeStatus::Success
}

fn player_proximity_check(npc: &mut Npc, _: &mut Commands, _: &QuerySet<(Entity, &Ball)>, player_query: &QuerySet<(Entity, &Player)>, _: &QuerySet<(Entity, &GameObject)>) -> NodeStatus {
    // Implement player proximity check logic
    NodeStatus::Success
}

fn set_tag_to_closest_ball(npc: &mut Npc, _: &mut Commands, ball_query: &QuerySet<(Entity, &Ball)>, _: &QuerySet<(Entity, &Player)>, _: &QuerySet<(Entity, &GameObject)>) -> NodeStatus {
    // Implement setting TAG to the closest ball logic
    NodeStatus::Success
}

fn set_tag_to_closest_object(npc: &mut Npc, _: &mut Commands, _: &QuerySet<(Entity, &Ball)>, _: &QuerySet<(Entity, &Player)>, object_query: &QuerySet<(Entity, &GameObject)>) -> NodeStatus {
    // Implement setting TAG to the closest object logic
    NodeStatus::Success
}

fn perform_a_star(npc: &mut Npc, _: &mut Commands, _: &QuerySet<(Entity, &Ball)>, _: &QuerySet<(Entity, &Player)>, _: &QuerySet<(Entity, &GameObject)>) -> NodeStatus {
    // Implement A* pathfinding logic
    NodeStatus::Success
}

fn swing(npc: &mut Npc, commands: &mut Commands, ball_query: &QuerySet<(Entity, &Ball)>, _: &QuerySet<(Entity, &Player)>, _: &QuerySet<(Entity, &GameObject)>) -> NodeStatus {
    // Implement swing logic
    NodeStatus::Success
}

// fn main() {
//     App::build()
//         .add_startup_system(setup.system())
//         .add_system(behavior_tree.system())
//         .add_plugin(BehaviorTreePlugin)
//         .run();
// }