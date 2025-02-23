use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::mpsc::Sender,
};

use crate::ThickMazeCell;

use super::{Algorithm, PathfindingAlgorithm, PathfindingResult, Point, MOVEMENTS};

#[derive(Default)]
pub struct BFS;

impl BFS {
    pub fn new() -> Self {
        BFS {}
    }

    fn reconstruct_path(came_from: &HashMap<Point, Point>, mut current: Point) -> Vec<Point> {
        let mut path = Vec::new();
        let mut visited = HashSet::new();

        while visited.insert(current) {
            if let Some(&prev) = came_from.get(&current) {
                path.push(current);
                current = prev;
            } else {
                break; // Break the loop if current point has no predecessor
            }
        }

        path.reverse();
        path
    }
}

impl PathfindingAlgorithm for BFS {
    fn find_path(&mut self, maze: &mut crate::maze::Maze, sender: &Sender<PathfindingResult>) {
        // Find entrance and exist coordinates
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

        let mut queue = VecDeque::new();
        let mut came_from: HashMap<Point, Point> = HashMap::new();

        queue.push_back(start);

        while let Some(current) = queue.pop_front() {
            maze.set_cell(current.x, current.y, ThickMazeCell::Visited);
            sender
                .send(PathfindingResult {
                    maze: maze.clone(),
                    stats: None,
                })
                .unwrap();

            if current == goal {
                // Reached the exit, reconstruct and visualize the path
                let path = Self::reconstruct_path(&came_from, current);
                for point in path.iter().skip(1) {
                    maze.set_cell(point.x, point.y, ThickMazeCell::FinalPath);

                    // Send the updated maze to the main thread
                    sender
                        .send(PathfindingResult {
                            maze: maze.clone(),
                            stats: None,
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
                    || maze.get_cell(neighbor.x, neighbor.y) == ThickMazeCell::Wall
                    || came_from.contains_key(&neighbor)
                {
                    continue;
                }

                came_from.insert(neighbor, current);
                queue.push_back(neighbor);
            }
        }
    }

    fn name(&self) -> super::Algorithm {
        Algorithm::BFS
    }

    fn get_stats(&self) -> Option<super::PathfindingStats> {
        None
    }
}
