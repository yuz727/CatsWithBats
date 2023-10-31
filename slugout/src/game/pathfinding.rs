use super::components::*;
use super::npc::{NPCBat, NPCFace, NPCTimer, NPCVelocity, States, NPC};
use crate::game::npc_events::*;
use crate::GameState;
use bevy::prelude::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Edge {
    coordinates: Vec2,
    cost: usize,
}

struct Vertex {
    coordinates: Vec2,
    cost: usize,
}



// map is a 2d vector of coordinates
// The whole map used for A* finna be fat
fn load_map() -> Vec<Vec<usize>> {
    let mut map: Vec<Vec<Edge>> = Vec::new();
    let mut curr_x = 0.;
    let mut curr_y;
    while curr_x <= 1280. {
        curr_y = 0.;
        let mut row: Vec<Edge> = Vec::new();
        while curr_y <= 720. {
            row.push(MAX);
            curr_y += 4.;
        }
        map.push(row);
        curr_x += 4.;
    }
    return map;
}

fn load_map() -> Vec<Vec<Vec2>> {
    let mut map: Vec<Vec<Edge>> = Vec::new();
    let mut curr_x = 0.;
    let mut curr_y;
    while curr_x <= 1280. {
        curr_y = 0.;
        let mut row: Vec<Edge> = Vec::new();
        while curr_y <= 720. {
            row.push(Vec2::new(0, 0));
            curr_y += 4.;
        }
        map.push(row);
        curr_x += 4.;
    }
    return map;
}

// Get ne
fn get_neighbours(map: Vec<Vec<Vec2>>, coords: Vec2) -> Vec<Vec2> {
    let mut ret = Vec::new();
    let x = coords.x as usize;
    let y = coords.y as usize;
    if x > 0 {
        ret.push(map[x - 1][y]);
    }
    // Each vertex is a 4x4 pixel grid
    if x < 320 {
        ret.push(map[x + 1][y]);
    }
    if y > 0 {
        ret.push(map[x][y - 1]);
    }
    if y < 180 {
        ret.push(map[x][y + 1]);
    }

    return ret;
}

fn manhattan_distance(a: Vec2, b: Vec2) -> f32 {
    return abs(a.x - b.x) + abs(a.y - b.y);
}

fn a_star(start: Vec2, goal: Vec2, &map_path: Vec<Vec<Vec2>>, &map_cost: Vec<Vec<usize>>) {
    let worklist = BinaryHeap::new();
    worlist.push(Reverse(Vertex{ start, 0 }));
    let mut current_path = map_path.clone();
    let mut current_cost = map_cost.clone();
    current_path[start.x][start.y] = -1;
    current_cost[start.x][start.y] = 0;

    while worklist.is_empty() == false {
        let currrent = worklist.pop();

        if (current.x == goal.x && current.y = goal.y) {
            break;
        }

        for neighbour in get_neighbours(map, current){
            let new_cost = current_cost[current.x][current.y] + 1;
            if current_cost[neighbour.x][neighbour.y] == -1 || new_cost < current_cost[neighbour.x][neighbour.y]{
                current_cost[neighbour.x][neighbour.y] = new_cost;
                let priority = new_cost + manhattan_distance(goal, neighbour);
                worklist.push(Reverse(Vertex{ neighbour, priority }));
                current_path[neighbour.x][neighbour.y] = current;
            }
        }
    }
}
