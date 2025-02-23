use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::mpsc::Sender,
};

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

        let mut queue = VecDeque::new();
        let mut came_from: HashMap<Point, Point> = HashMap::new();

        queue.push_back(entrance);

        while let Some(current) = queue.pop_front() {
            maze.mark_cell_as_visited(current);
            sender
                .send(PathfindingResult {
                    maze: maze.clone(),
                    stats: None,
                })
                .unwrap();

            if current == exit {
                // Reached the exit, reconstruct and visualize the path
                let path = Self::reconstruct_path(&came_from, current);
                for point in path.iter().skip(1) {
                    maze.mark_cell_as_final_path(*point);

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

                if !maze.is_valid_coord(neighbor.x as i32, neighbor.y as i32)
                    || maze.is_not_passable(current, neighbor)
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
