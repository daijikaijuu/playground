use std::{thread, time::Duration};

use rand::seq::SliceRandom;

use crate::{maze::MazeCell, visualization::MazeVisualization};

pub trait PathfindingAlgorithm {
    fn find_path(&mut self, visualizer: &mut MazeVisualization) -> bool;
}

pub struct Backtracking {
    pub visualization_delay: u64,
}

impl Backtracking {
    pub fn new(visualization_delay: u64) -> Self {
        Backtracking {
            visualization_delay,
        }
    }

    fn backtrack(
        &mut self,
        visualization: &mut MazeVisualization,
        x: usize,
        y: usize,
        exit_x: usize,
        exit_y: usize,
    ) -> bool {
        visualization.maze.set_cell(x, y, MazeCell::Visited); // Mark cell as visited

        // If we've reached the exit, stop recursion
        if x == exit_x && y == exit_y {
            return true;
        }

        let directions = &[(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut rng = rand::thread_rng();
        let shuffled_directions = directions.choose_multiple(&mut rng, directions.len());

        for &(dx, dy) in shuffled_directions {
            let new_x: i32 = x as i32 + dx;
            let new_y: i32 = y as i32 + dy;

            if visualization.maze.is_valid_move(new_x as i32, new_y as i32)
                && (visualization.maze.get_cell(new_x as usize, new_y as usize) == MazeCell::Path
                    || visualization.maze.get_cell(new_x as usize, new_y as usize)
                        == MazeCell::Exit)
                && !visualization.rl.window_should_close()
            {
                // Mark the final path
                visualization.maze.set_cell(x, y, MazeCell::Visited);
                visualization.draw();
                thread::sleep(Duration::from_millis(self.visualization_delay));
                // Mark the path recursively backtrack
                if self.backtrack(
                    visualization,
                    new_x as usize,
                    new_y as usize,
                    exit_x,
                    exit_y,
                ) {
                    return true;
                }
            }
        }

        false
    }
}

impl PathfindingAlgorithm for Backtracking {
    fn find_path(&mut self, visualization: &mut MazeVisualization) -> bool {
        // Reset the maze to its original state
        // Find entrance and exit coordinated
        let entrance = visualization.maze.get_entrance().unwrap();
        let exit = visualization.maze.get_exit().unwrap();

        // Start the backtracking algorithm from the entrance
        self.backtrack(visualization, entrance.0, entrance.1, exit.0, exit.1)
    }
}
