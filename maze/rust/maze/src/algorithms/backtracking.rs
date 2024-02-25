use std::sync::mpsc::Sender;

use rand::seq::SliceRandom;

use crate::maze::{Maze, MazeCell};

use super::{pathfinding::PathfindingAlgorithm, Algorithm, MOVEMENTS};

pub struct Backtracking {}

impl Backtracking {
    pub fn new() -> Self {
        Backtracking {}
    }

    fn backtrack(
        maze: &mut Maze,
        sender: &Sender<Maze>,
        x: usize,
        y: usize,
        exit_x: usize,
        exit_y: usize,
    ) -> bool {
        maze.set_cell(x, y, MazeCell::Visited); // Mark cell as visited

        // If we've reached the exit, stop recursion
        if x == exit_x && y == exit_y {
            sender.send(maze.clone()).unwrap();
            return true;
        }

        let directions = &MOVEMENTS;
        let mut rng = rand::thread_rng();
        let shuffled_directions = directions.choose_multiple(&mut rng, directions.len());

        for &(dx, dy) in shuffled_directions {
            let new_x: i32 = x as i32 + dx;
            let new_y: i32 = y as i32 + dy;

            if maze.is_valid_move(new_x as i32, new_y as i32)
                && (maze.get_cell(new_x as usize, new_y as usize) == MazeCell::Path
                    || maze.get_cell(new_x as usize, new_y as usize) == MazeCell::Exit)
            {
                sender.send(maze.clone()).unwrap();

                // Mark the final path
                maze.set_cell(x, y, MazeCell::FinalPath);
                // Mark the path recursively backtrack
                if Backtracking::backtrack(
                    maze,
                    sender,
                    new_x as usize,
                    new_y as usize,
                    exit_x,
                    exit_y,
                ) {
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
    fn find_path(&mut self, maze: &mut Maze, sender: &Sender<Maze>) {
        // Reset the maze to its original state
        // Find entrance and exit coordinated
        let entrance = maze.get_entrance().expect("Cannot find entrance");
        let exit = maze.get_exit().expect("Cannot find exit");

        Backtracking::backtrack(maze, sender, entrance.0, entrance.1, exit.0, exit.1);
    }

    fn name(&self) -> Algorithm {
        Algorithm::Backtracking
    }
}
