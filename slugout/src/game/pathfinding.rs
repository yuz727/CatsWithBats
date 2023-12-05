use crate::game::npc::*;
use bevy::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

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

// // For min-heap implementation
impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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

pub fn load_walkable_map_1() -> Vec<Vec<bool>> {
    let mut walkable_map: Vec<Vec<bool>> = Vec::new();
    //  let mut records = reader.records();
    let mut curr_x = 0.;
    let mut curr_y;
    while curr_x < 1280. {
        curr_y = 0.;
        let mut row: Vec<bool> = Vec::new();
        while curr_y < 720. {
            if (curr_x >= 708. && curr_x <= 812.) && (curr_y <= 244. && curr_y >= 136.) {
                row.push(false);
                curr_y += 4.;
                continue;
            }
            if (curr_x >= 556. && curr_x < 724.) && (curr_y <= 664. && curr_y >= 544.) {
                row.push(false);
                curr_y += 4.;
                continue;
            }
            if (curr_x >= 524. && curr_x <= 636.) && (curr_y <= 244. && curr_y >= 52.) {
                row.push(false);
                curr_y += 4.;
                continue;
            }
            row.push(true);
            curr_y += 4.;
            continue;
        }
        walkable_map.push(row);
        curr_x += 4.;
    }
    return walkable_map;
}

pub fn load_walkable_map_no_objects() -> Vec<Vec<bool>> {
    let mut walkable_map: Vec<Vec<bool>> = Vec::new();
    //  let mut records = reader.records();
    let mut curr_x = 0.;
    let mut curr_y;
    while curr_x < 1280. {
        curr_y = 0.;
        let mut row: Vec<bool> = Vec::new();
        while curr_y < 720. {
            row.push(true);
            curr_y += 4.;
        }
        walkable_map.push(row);
        curr_x += 4.;
    }
    return walkable_map;
}

pub fn load_walkable_map_4() {}

/*  Return a vector for the neighbouring tiles of a given tile
 */
fn get_neighbours(map: &Vec<Vec<Vec2>>, coords: Vec2) -> Vec<Vec2> {
    let mut ret = Vec::new();
    let x = coords.x as usize / 4;
    let y = coords.y as usize / 4;
    if x > 0 {
        if y > 0 {
            ret.push(map[x - 1][y - 1]);
            ret.push(map[x][y - 1]);
        }
        ret.push(map[x - 1][y]);
        if y < 179 {
            ret.push(map[x - 1][y + 1]);
            ret.push(map[x][y + 1]);
        }
    }
    if x < 319 {
        if y > 0 {
            ret.push(map[x + 1][y - 1]);
        }
        ret.push(map[x + 1][y]);
        if y < 179 {
            ret.push(map[x + 1][y + 1]);
        }
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
            if maps.walkable[neighbourx][neighboury] == false {
                // Try to avoid objects in the map
                continue;
            }
            //  info!("neightbour");
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

    // No path found, just return an empty vector
    if current.x != goal_x || current.y != goal_y {
        return Vec::with_capacity(1);
    }

    // Generate the path and store it into a vector
    // Reverse the order so it is start to goal, then return result
    let mut ret: Vec<Vec2> = Vec::new();
    while current.x != start.x || current.y != start.y {
        ret.push(coords_conversion_bevy(current));
        current = current_path[current.x as usize / 4][current.y as usize / 4];
    }

    ret.push(coords_conversion_bevy(start));
    return ret;
}

pub fn coords_conversion_astar(coord: Vec2) -> Vec2 {
    return Vec2::new(coord.x + 640., 360. - coord.y);
}
fn coords_conversion_bevy(coord: Vec2) -> Vec2 {
    return Vec2::new(coord.x - 640., 0. - (coord.y - 360.));
}
