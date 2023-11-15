use crate::game::npc::*;
use bevy::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::path;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Vertex {
    x: usize,
    y: usize,
    cost: i32,
}

// For min-heap implementation
impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

// For min-heap implementation
impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
/*  Create a 2-d vector of tiles, each tile has a tile with a cost associated to it, default at max-int
 *  Each tile will be a 4x4 pixel chunk
 */
pub fn load_map_cost() -> Vec<Vec<i32>> {
    let mut cost_map: Vec<Vec<i32>> = Vec::new();
    let mut curr_x = 0.;
    let mut curr_y;
    while curr_x < 1280. {
        curr_y = 0.;
        let mut row: Vec<i32> = Vec::new();
        while curr_y < 720. {
            row.push(-1);
            curr_y += 4.;
        }
        cost_map.push(row);
        curr_x += 4.;
    }
    return cost_map;
}

/*  Create a 2-d vector of tiles, each tile has the actual coordinates associated to it
 *  Each tile will be a 4x4 pixel chunk
 */
pub fn load_map_path() -> Vec<Vec<Vec2>> {
    let mut path_map: Vec<Vec<Vec2>> = Vec::new();
    let mut curr_x = 0.;
    let mut curr_y;
    while curr_x < 1280. {
        curr_y = 0.;
        let mut row: Vec<Vec2> = Vec::new();
        while curr_y < 720. {
            row.push(Vec2::new(curr_x, curr_y));
            curr_y += 4.;
        }
        path_map.push(row);
        curr_x += 4.;
    }
    return path_map;
}

/*  Return a vector for the neighbouring tiles of a given tile
 */
fn get_neighbours(map: &Vec<Vec<Vec2>>, coords: Vec2) -> Vec<Vec2> {
    let mut ret = Vec::new();
    let x = coords.x as usize / 4;
    let y = coords.y as usize / 4;
    if x > 0 {
        ret.push(map[x - 1][y]);
    }
    if y < 179 {
        ret.push(map[x][y + 1]);
    }
    if x < 319 {
        ret.push(map[x + 1][y]);
    }
    if y > 0 {
        ret.push(map[x][y - 1]);
    }

    return ret;
}

/*  Returns the Manhattan Distance between two points
 */
fn manhattan_distance(a: Vec2, b: Vec2) -> f32 {
    return (a.x - b.x).abs() + (a.y - b.y).abs();
}

pub fn a_star(start: Vec2, goal: Vec2, maps: &Maps) -> Vec<Vec2> {
    // Initialise
    let mut worklist = BinaryHeap::new();
    worklist.push(Vertex {
        x: start.x as usize,
        y: start.y as usize,
        cost: 0,
    });
    let goal_x = goal.x - (goal.x % 4.);
    let goal_y = goal.y - (goal.y % 4.);
    // Initialise data structures needed
    let mut current_path = [[Vec2::new(-1., -1.); 180]; 320];
    let mut current_cost: [[i32; 180]; 320] = [[-1; 180]; 320];
    let mut current = Vec2::new(-1., -1.);
    // Main algorithm loop
    while let Some(Vertex { x, y, cost: _ }) = worklist.pop() {
        // the current.x and current.y will be the actual coordinates in ghr game world
        current = Vec2::new(x as f32, y as f32); // Take the first vertex in the min-heaps
        if (current.x == goal_x) && (current.y == goal_y) {
            // Break out of loop when the goal is reached
            break;
        }
        let currentx = current.x as usize / 4;
        let currenty = current.y as usize / 4;
        for neighbour in get_neighbours(&maps.path_map, current) {
            // Get a neighbouring tile, covert the coordinates to index in array/vector
            let neighbourx = neighbour.x as usize / 4;
            let neighboury = neighbour.y as usize / 4;
            let new_cost: i32 = current_cost[currentx][currenty] + 1;
            if current_cost[neighbourx][neighboury] == -1
                || new_cost < current_cost[neighbourx][neighboury]
            {
                current_cost[neighbourx][neighboury] = new_cost;
                worklist.push(Vertex {
                    x: neighbourx * 4,
                    y: neighboury * 4,
                    cost: (new_cost as f32 + manhattan_distance(goal, neighbour)) as i32,
                });
                current_path[neighbourx][neighboury] = current;
            }
        }
    }
    info!(goal_x);
    info!(goal_y);
    // No path found, just return a vector containing 0
    if current.x != goal_x || current.y != goal_y {
        return vec![Vec2::ZERO];
    }

    // Generate the path and store it into a vector
    // Reverse the order so it is start to goal, then return result
    let mut ret: Vec<Vec2> = Vec::new();
    while current.x != start.x || current.y != start.y {
        ret.push(current);
        current = current_path[current.x as usize / 4][current.y as usize / 4];
    }

    ret.push(start);
    ret.reverse();
    return ret;
}
