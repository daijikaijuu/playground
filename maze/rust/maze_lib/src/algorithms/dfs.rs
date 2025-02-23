use rand::{rngs::ThreadRng, seq::SliceRandom};
use std::{collections::HashSet, sync::mpsc::Sender};

use crate::{maze::Maze, ThickMazeCell};

use super::{
    Algorithm, MazeGenerationAlgorithm, PathfindingAlgorithm, PathfindingResult, PathfindingStats,
    Point, MOVEMENTS, MOVEMENTS_X2,
};

#[derive(Default, Copy, Clone)]
pub struct DFS {
    stats: PathfindingStats,
}

impl DFS {
    pub fn new() -> Self {
        DFS {
            stats: PathfindingStats::default(),
        }
    }

    fn depth_first_search(
        &mut self,
        current: Point,
        goal: Point,
        maze: &mut Maze,
        sender: &Sender<PathfindingResult>,
        visited: &mut HashSet<Point>,
    ) -> bool {
        visited.insert(current);
        maze.set_cell(current.x, current.y, ThickMazeCell::FinalPath);
        self.stats.new_step();

        if current == goal {
            return true;
        }

        for (dx, dy) in MOVEMENTS {
            let neighbor = Point {
                x: (current.x as i32 + dx) as usize,
                y: (current.y as i32 + dy) as usize,
            };

            if maze.is_valid_move(neighbor.x as i32, neighbor.y as i32)
                && !visited.contains(&Point {
                    x: neighbor.x,
                    y: neighbor.y,
                })
                && maze.get_cell(neighbor.x, neighbor.y) != ThickMazeCell::Wall
            {
                sender
                    .send(PathfindingResult {
                        maze: maze.clone(),
                        stats: None,
                    })
                    .unwrap();

                // Mark the final path
                maze.set_cell(neighbor.x, neighbor.y, ThickMazeCell::FinalPath);

                if self.depth_first_search(
                    Point {
                        x: neighbor.x,
                        y: neighbor.y,
                    },
                    goal,
                    maze,
                    sender,
                    visited,
                ) {
                    return true;
                } else {
                    visited.remove(&neighbor);
                    maze.set_cell(neighbor.x, neighbor.y, ThickMazeCell::Visited);
                }
            }
        }

        false
    }

    fn depth_first_maze_generation(current: Point, maze: &mut Maze, rng: &mut ThreadRng) -> bool {
        let mut shuffled_directions = MOVEMENTS_X2.to_vec();
        shuffled_directions.shuffle(rng);
        for &(dx, dy) in &shuffled_directions {
            let new_x = current.x as i32 + dx;
            let new_y = current.y as i32 + dy;

            if maze.is_valid_move(new_x, new_y) {
                let nx = new_x as usize;
                let ny = new_y as usize;

                if maze.get_cell(nx, ny) == ThickMazeCell::Wall {
                    maze.set_cell(nx, ny, ThickMazeCell::Path);
                    maze.set_cell(
                        (current.x + nx) / 2,
                        (current.y + ny) / 2,
                        ThickMazeCell::Path,
                    );

                    DFS::depth_first_maze_generation(Point { x: nx, y: ny }, maze, rng);
                }
            }
        }
        false
    }
}

impl PathfindingAlgorithm for DFS {
    fn find_path(&mut self, maze: &mut Maze, sender: &Sender<PathfindingResult>) {
        let entrance = maze.get_entrance().expect("Cannot find entrance");
        let exit = maze.get_exit().expect("Cannot find exit");

        let mut visited = HashSet::new();
        self.depth_first_search(
            Point {
                x: entrance.0,
                y: entrance.1,
            },
            Point {
                x: exit.0,
                y: exit.1,
            },
            maze,
            sender,
            &mut visited,
        );
    }

    fn name(&self) -> super::Algorithm {
        Algorithm::DFS
    }

    fn get_stats(&self) -> Option<PathfindingStats> {
        None
    }
}

impl MazeGenerationAlgorithm for DFS {
    fn generate(
        &mut self,
        width: usize,
        height: usize,
        start_x: usize,
        start_y: usize,
    ) -> Option<Maze> {
        let mut maze = Maze::new(width, height);
        let mut rng = rand::thread_rng();

        maze.set_cell(start_x, start_y, ThickMazeCell::Path);
        DFS::depth_first_maze_generation(
            Point {
                x: start_x,
                y: start_y,
            },
            &mut maze,
            &mut rng,
        );

        maze.set_cell(start_x, start_y, ThickMazeCell::Entrance);
        let exit_point = maze.get_random_boundary_point(&mut rng);
        maze.set_cell(exit_point.0, exit_point.1, ThickMazeCell::Exit);

        maze.backup();

        Some(maze)
    }
}
