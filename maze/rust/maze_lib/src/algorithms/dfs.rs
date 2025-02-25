use rand::{rngs::ThreadRng, seq::SliceRandom};
use std::{collections::HashSet, sync::mpsc::Sender};

use crate::{maze::Maze, CellType, MazeType};

use super::{
    Algorithm, MazeGenerationAlgorithm, Movements, PathfindingAlgorithm, PathfindingResult,
    PathfindingStats, Point,
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
        maze.mark_cell_as_final_path(current);
        self.stats.new_step();

        if current == goal {
            return true;
        }

        for (dx, dy) in &Movements::directions() {
            let neighbor = Point {
                x: (current.x as i32 + dx) as usize,
                y: (current.y as i32 + dy) as usize,
            };

            if maze.is_valid_coord(neighbor.x as i32, neighbor.y as i32)
                && !visited.contains(&Point {
                    x: neighbor.x,
                    y: neighbor.y,
                })
                && maze.is_passable(current, neighbor)
            {
                sender
                    .send(PathfindingResult {
                        maze: maze.clone(),
                        stats: None,
                    })
                    .unwrap();

                // Mark the final path
                maze.mark_cell_as_final_path(neighbor);

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
                    maze.mark_cell_as_visited(neighbor);
                }
            }
        }

        false
    }

    fn depth_first_maze_generation(current: Point, maze: &mut Maze, rng: &mut ThreadRng) -> bool {
        let mut shuffled_directions = Movements::directions_doubled().to_vec();
        shuffled_directions.shuffle(rng);
        for &(dx, dy) in &shuffled_directions {
            let new_x = current.x as i32 + dx;
            let new_y = current.y as i32 + dy;

            if maze.is_valid_coord(new_x, new_y) {
                let neighbor = Point {
                    x: new_x as usize,
                    y: new_y as usize,
                };

                if maze.is_not_passable(current, neighbor) {
                    maze.mark_cell_as_path(neighbor);
                    maze.mark_cell_as_path(Point {
                        x: (current.x + neighbor.x) / 2,
                        y: (current.y + neighbor.y) / 2,
                    });

                    DFS::depth_first_maze_generation(neighbor, maze, rng);
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
        self.depth_first_search(entrance, exit, maze, sender, &mut visited);
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
        maze_type: MazeType,
        width: usize,
        height: usize,
        entrance: Point,
    ) -> Option<Maze> {
        let mut maze = Maze::new(width, height, maze_type, Some(CellType::Wall));
        let mut rng = rand::thread_rng();

        maze.mark_cell_as_path(entrance);
        DFS::depth_first_maze_generation(entrance, &mut maze, &mut rng);

        maze.mark_cell_as_entrance(entrance);
        let exit_point = maze.get_random_boundary_point(&mut rng);
        maze.mark_cell_as_exit(exit_point);

        maze.backup();

        Some(maze)
    }
}
