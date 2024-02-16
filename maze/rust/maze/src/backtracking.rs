use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use rand::seq::SliceRandom;

use crate::{
    maze::{Maze, MazeCell},
    pathfinding::PathfindingAlgorithm,
    visualization::MazeVisualization,
};

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

        let directions = &[(0, 1), (1, 0), (0, -1), (-1, 0)];
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
    fn find_path(&mut self, visualization: &mut MazeVisualization) -> bool {
        let (sender, receiver): (Sender<Maze>, Receiver<Maze>) = channel();
        let maze = Arc::new(Mutex::new(visualization.maze.clone()));

        // Reset the maze to its original state
        // Find entrance and exit coordinated
        let entrance = visualization.maze.get_entrance().unwrap();
        let exit = visualization.maze.get_exit().unwrap();

        // Clone the initial maze information and send it to the main thread
        match sender
            .send(visualization.maze.clone())
            .map_err(|e| format!("Failed to send initial data: {}", e))
        {
            Ok(it) => it,
            Err(_err) => return false,
        };

        let handle = thread::spawn(move || {
            let mut maze = maze.lock().unwrap();

            // Start the backtracking algorithm from the entrance
            Backtracking::backtrack(&mut *maze, &sender, entrance.0, entrance.1, exit.0, exit.1);
        });

        while let Ok(recieved_maze) = receiver.try_recv() {
            if visualization.rl.window_should_close() {}
            // Update the visualization with the new maze
            visualization.set_maze(&recieved_maze);

            visualization.visualize(self.name());

            thread::sleep(Duration::from_millis(30));
        }

        // Wait for the backtracking thread to finish
        handle.join().unwrap();

        true
    }

    fn name(&self) -> &str {
        "Backtracking"
    }
}
