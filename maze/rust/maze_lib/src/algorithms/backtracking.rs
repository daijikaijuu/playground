use std::collections::HashSet;
use std::sync::mpsc::Sender;

use rand::seq::SliceRandom;

use crate::{
    algorithms::MOVEMENTS_X2,
    maze::{Maze, MazeCell},
};

use super::{
    pathfinding::PathfindingAlgorithm, Algorithm, MazeGenerationAlgorithm, PathfindingResult,
    PathfindingStats, Point, MOVEMENTS,
};

#[derive(Default)]
pub struct Backtracking {
    stats: PathfindingStats,
}

impl Backtracking {
    pub fn new() -> Self {
        Backtracking {
            stats: PathfindingStats::default(),
        }
    }

    fn backtrack(
        &mut self,
        maze: &mut Maze,
        sender: &Sender<PathfindingResult>,
        x: usize,
        y: usize,
        exit_x: usize,
        exit_y: usize,
    ) -> bool {
        maze.set_cell(x, y, MazeCell::Visited); // Mark cell as visited

        // Update stats
        self.stats.new_step();

        // If we've reached the exit, stop recursion
        if x == exit_x && y == exit_y {
            sender
                .send(PathfindingResult {
                    maze: maze.clone(),
                    stats: Some(self.stats),
                })
                .unwrap();
            return true;
        }

        let directions = &MOVEMENTS;
        let mut rng = rand::thread_rng();
        let shuffled_directions = directions.choose_multiple(&mut rng, directions.len());

        for &(dx, dy) in shuffled_directions {
            let new_x: i32 = x as i32 + dx;
            let new_y: i32 = y as i32 + dy;

            if maze.is_valid_move(new_x, new_y)
                && (maze.get_cell(new_x as usize, new_y as usize) == MazeCell::Path
                    || maze.get_cell(new_x as usize, new_y as usize) == MazeCell::Exit)
            {
                sender
                    .send(PathfindingResult {
                        maze: maze.clone(),
                        stats: Some(self.stats),
                    })
                    .unwrap();

                // Mark the final path
                maze.set_cell(x, y, MazeCell::FinalPath);
                // Mark the path recursively backtrack
                if self.backtrack(maze, sender, new_x as usize, new_y as usize, exit_x, exit_y) {
                    return true;
                } else {
                    maze.set_cell(x, y, MazeCell::Visited);
                }
            }
        }

        false
    }
}

impl PathfindingAlgorithm for Backtracking {
    fn find_path(&mut self, maze: &mut Maze, sender: &Sender<PathfindingResult>) {
        // Reset the maze to its original state
        // Find entrance and exit coordinated
        let entrance = maze.get_entrance().expect("Cannot find entrance");
        let exit = maze.get_exit().expect("Cannot find exit");

        self.backtrack(maze, sender, entrance.0, entrance.1, exit.0, exit.1);
    }

    fn name(&self) -> Algorithm {
        Algorithm::Backtracking
    }

    fn get_stats(&self) -> Option<PathfindingStats> {
        Some(self.stats)
    }
}

impl MazeGenerationAlgorithm for Backtracking {
    fn generate(
        &mut self,
        width: usize,
        height: usize,
        start_x: usize,
        start_y: usize,
    ) -> Option<Maze> {
        let mut maze = Maze::new(width, height);
        let mut rng = rand::thread_rng();
        let mut visited = HashSet::new();

        fn generate_maze_recursive(
            current: Point,
            maze: &mut Maze,
            visited: &mut HashSet<Point>,
            rng: &mut impl rand::Rng,
        ) {
            visited.insert(current);
            maze.set_cell(current.x, current.y, MazeCell::Path);

            let mut directions = MOVEMENTS_X2.to_vec();
            directions.shuffle(rng);

            for (dx, dy) in directions {
                let new_x = current.x as i32 + dx;
                let new_y = current.y as i32 + dy;

                if maze.is_valid_move(new_x, new_y) {
                    let next = Point {
                        x: new_x as usize,
                        y: new_y as usize,
                    };

                    if !visited.contains(&next) {
                        // Carve path between current and next
                        maze.set_cell(
                            (current.x + next.x) / 2,
                            (current.y + next.y) / 2,
                            MazeCell::Path,
                        );
                        generate_maze_recursive(next, maze, visited, rng);
                    }
                }
            }
        }

        let start = Point {
            x: start_x,
            y: start_y,
        };
        generate_maze_recursive(start, &mut maze, &mut visited, &mut rng);

        // Set entrance and exit
        maze.set_cell(start_x, start_y, MazeCell::Entrance);
        let exit_point = maze.get_random_boundary_point(&mut rng);
        maze.set_cell(exit_point.0, exit_point.1, MazeCell::Exit);

        maze.backup();
        Some(maze)
    }
}
