use crate::maze::{Maze, MazeCell, MazeGenerator};
use raylib::prelude::*;

pub struct MazeVisualization<'a> {
    pub maze: Maze,
    pub rl: &'a mut RaylibHandle,
    thread: &'a RaylibThread,
}

impl<'a> MazeVisualization<'a> {
    pub fn new(
        width: usize,
        height: usize,
        rl: &'a mut RaylibHandle,
        thread: &'a RaylibThread,
    ) -> Self {
        let mut maze = Maze::new(width, height);
        maze.generate_maze(1, 1);
        MazeVisualization { maze, rl, thread }
    }

    pub fn draw(&mut self, title: &str) {
        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::BLACK);

        // Calculate the new cell size based on window size and padding
        let new_cell_size = ((d.get_screen_width() - 35) as f32 / self.maze.width as f32) as i32;

        // Calculate center-aligned position for the algorithm name
        let text_size = raylib::core::text::measure_text(&title, 20);

        let center_x = (d.get_screen_width() - text_size) / 2;
        let center_y = 10;

        // Draw the pathfinding algorithm name above the maze
        d.draw_text(&title, center_x, center_y, 20, Color::WHITE);

        for y in 0..self.maze.height {
            for x in 0..self.maze.width {
                let color = match self.maze.get_cell(x, y) {
                    MazeCell::Wall => Color::LIGHTGRAY,
                    MazeCell::Path => Color::BLACK,
                    MazeCell::Entrance => Color::BLUE,
                    MazeCell::Exit => Color::RED,
                    MazeCell::Visited => Color::DARKBLUE,
                    MazeCell::FinalPath => Color::GREEN,
                };

                d.draw_rectangle(
                    35 + x as i32 * new_cell_size,
                    35 + y as i32 * new_cell_size,
                    new_cell_size,
                    new_cell_size,
                    color,
                );
            }
        }
    }

    pub fn visualize(&mut self, title: &str) {
        self.draw(&title);
    }

    pub fn set_maze(&mut self, maze: &Maze) {
        self.maze = maze.clone();
    }
}
