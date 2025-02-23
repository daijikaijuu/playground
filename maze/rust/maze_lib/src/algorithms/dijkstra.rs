use std::{
    collections::{BinaryHeap, HashMap},
    sync::mpsc::Sender,
};

use crate::maze::Maze;

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

#[derive(Default)]
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

        let mut open_set = BinaryHeap::new();
        let mut came_from: HashMap<Point, Point> = HashMap::new();
        let mut costs: HashMap<Point, u32> = HashMap::new();

        open_set.push(Node {
            point: entrance,
            cost: 0,
        });
        costs.insert(entrance, 0);

        while let Some(current_node) = open_set.pop() {
            let current = current_node.point;
            maze.mark_cell_as_visited(current);
            sender
                .send(PathfindingResult {
                    stats: None,
                    maze: maze.clone(),
                })
                .unwrap();

            if current == exit {
                // Reached the exit, reconstruct and visualize the path
                let path = Self::reconstruct_path(&came_from, current);
                for point in path.iter().skip(1) {
                    maze.mark_cell_as_final_path(*point);

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

                if !maze.is_valid_coord(neighbor.x as i32, neighbor.y as i32)
                    || maze.is_not_passable(current, neighbor)
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
