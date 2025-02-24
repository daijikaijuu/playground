use std::collections::HashSet;
use std::sync::mpsc::Sender;

use rand::seq::SliceRandom;

use crate::{maze::Maze, MazeType};

use super::{
    pathfinding::PathfindingAlgorithm, Algorithm, MazeGenerationAlgorithm, Movements,
    PathfindingResult, PathfindingStats, Point,
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
        current: Point,
        exit: Point,
    ) -> bool {
        maze.mark_cell_as_visited(current);

        // Update stats
        self.stats.new_step();

        // If we've reached the exit, stop recursion
        if current == exit {
            sender
                .send(PathfindingResult {
                    maze: maze.clone(),
                    stats: Some(self.stats),
                })
                .unwrap();
            return true;
        }

        let directions = &Movements::directions();
        let mut rng = rand::thread_rng();
        let shuffled_directions = directions.choose_multiple(&mut rng, directions.len());

        for &(dx, dy) in shuffled_directions {
            let new_x: i32 = current.x as i32 + dx;
            let new_y: i32 = current.y as i32 + dy;
            let neighbor = Point {
                x: new_x as usize,
                y: new_y as usize,
            };

            if maze.is_valid_coord(new_x, new_y)
                && (maze.is_passable(current, neighbor) || neighbor == exit)
            {
                sender
                    .send(PathfindingResult {
                        maze: maze.clone(),
                        stats: Some(self.stats),
                    })
                    .unwrap();

                // Mark the final path
                maze.mark_cell_as_final_path(current);
                // Mark the path recursively backtrack
                if self.backtrack(
                    maze,
                    sender,
                    Point {
                        x: new_x as usize,
                        y: new_y as usize,
                    },
                    exit,
                ) {
                    return true;
                } else {
                    maze.mark_cell_as_visited(current);
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

        self.backtrack(maze, sender, entrance, exit);
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
        maze_type: MazeType,
        width: usize,
        height: usize,
        entrance: Point,
    ) -> Option<Maze> {
        let mut maze = Maze::new(width, height, maze_type, None);
        let mut rng = rand::thread_rng();
        let mut visited = HashSet::new();

        fn generate_maze_recursive(
            current: Point,
            maze: &mut Maze,
            visited: &mut HashSet<Point>,
            rng: &mut impl rand::Rng,
        ) {
            visited.insert(current);
            maze.mark_cell_as_path(current);

            let mut directions = Movements::directions_doubled().to_vec();
            directions.shuffle(rng);

            for (dx, dy) in directions {
                let new_x = current.x as i32 + dx;
                let new_y = current.y as i32 + dy;

                if maze.is_valid_coord(new_x, new_y) {
                    let next = Point {
                        x: new_x as usize,
                        y: new_y as usize,
                    };

                    if !visited.contains(&next) {
                        // Carve path between current and next
                        maze.mark_cell_as_path(Point {
                            x: (current.x + next.x) / 2,
                            y: (current.y + next.y) / 2,
                        });
                        generate_maze_recursive(next, maze, visited, rng);
                    }
                }
            }
        }

        generate_maze_recursive(entrance, &mut maze, &mut visited, &mut rng);

        // Set entrance and exit
        maze.mark_cell_as_entrance(entrance);
        let exit_point = maze.get_random_boundary_point(&mut rng);
        maze.mark_cell_as_exit(exit_point);

        maze.backup();
        Some(maze)
    }
}
