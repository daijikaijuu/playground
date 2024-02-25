use std::{
    collections::{BinaryHeap, HashMap},
    sync::mpsc::Sender,
};

use crate::maze::{Maze, MazeCell};

use super::{Algorithm, PathfindingAlgorithm, PathfindingResult, Point, MOVEMENTS};

#[derive(Clone, Copy, PartialEq, Eq)]
struct Node {
    point: Point,
    cost: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Dijkstra;

impl Dijkstra {
    pub fn new() -> Self {
        Dijkstra
    }
}

impl Dijkstra {
    fn reconstruct_path(came_from: &HashMap<Point, Point>, mut current: Point) -> Vec<Point> {
        let mut path = vec![current];
        while came_from.contains_key(&current) {
            current = *came_from.get(&current).unwrap();
            path.push(current);
        }
        path.reverse();
        path
    }
}

impl PathfindingAlgorithm for Dijkstra {
    fn find_path(&mut self, maze: &mut Maze, sender: &Sender<PathfindingResult>) {
        // Find entrance and exit coordinates
        let entrance = maze.get_entrance().expect("Cannot find entrance point");
        let exit = maze.get_exit().expect("Cannot find exit point");

        let start = Point {
            x: entrance.0,
            y: entrance.1,
        };
        let goal = Point {
            x: exit.0,
            y: exit.1,
        };

        let mut open_set = BinaryHeap::new();
        let mut came_from: HashMap<Point, Point> = HashMap::new();
        let mut costs: HashMap<Point, u32> = HashMap::new();

        open_set.push(Node {
            point: start,
            cost: 0,
        });
        costs.insert(start, 0);

        while let Some(current_node) = open_set.pop() {
            let current = current_node.point;
            maze.set_cell(current.x, current.y, MazeCell::Visited);
            sender
                .send(PathfindingResult {
                    stats: None,
                    maze: maze.clone(),
                })
                .unwrap();

            if current == goal {
                // Reached the exit, reconstruct and visualize the path
                let path = Self::reconstruct_path(&came_from, current);
                for point in path.iter().skip(1) {
                    maze.set_cell(point.x, point.y, MazeCell::FinalPath);

                    // Send the updated maze to the mazin thread
                    sender
                        .send(PathfindingResult {
                            stats: None,
                            maze: maze.clone(),
                        })
                        .expect("Failed to send maze to the main thread");
                }
                break;
            }

            for (dx, dy) in &MOVEMENTS {
                let neighbor = Point {
                    x: (current.x as i32 + dx) as usize,
                    y: (current.y as i32 + dy) as usize,
                };

                if !maze.is_valid_move(neighbor.x as i32, neighbor.y as i32)
                    || maze.get_cell(neighbor.x, neighbor.y) == MazeCell::Wall
                {
                    continue;
                }

                let tentative_cost = costs[&current] + 1;

                if !costs.contains_key(&neighbor) || tentative_cost < costs[&neighbor] {
                    costs.insert(neighbor, tentative_cost);
                    came_from.insert(neighbor, current);

                    open_set.push(Node {
                        point: neighbor,
                        cost: tentative_cost,
                    })
                }
            }
        }
    }

    fn name(&self) -> super::Algorithm {
        Algorithm::Dijkstra
    }

    fn get_stats(&self) -> Option<super::PathfindingStats> {
        None
    }
}
