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
        println!("{:?}", maze);
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
                    MazeCell::Visited => Color::GREEN,
                    MazeCell::FinalPath => Color::GOLD,
                    MazeCell::WeightedPath(weight) => {
                        // Choose a color based on the weight, you can adjust this logic
                        Color::new(
                            (weight % 256) as u8,
                            ((weight / 256) % 256) as u8,
                            ((weight / 256 / 256) % 256) as u8,
                            255,
                        )
                    }
                };

                d.draw_rectangle(
                    35 + x as i32 * new_cell_size,
                    35 + y as i32 * new_cell_size,
                    new_cell_size,
                    new_cell_size,
                    color,
                );

                // Draw weight information inside the cell
                if let MazeCell::WeightedPath(weight) = self.maze.get_cell(x, y) {
                    let text = format!("{}", weight);
                    let text_size = raylib::core::text::measure_text(&text, 10);

                    d.draw_text(
                        &text,
                        x as i32 * new_cell_size + new_cell_size / 2 - text_size / 2,
                        y as i32 * new_cell_size + new_cell_size / 2 - text_size / 2,
                        10,
                        Color::RED,
                    );
                }
            }
        }
    }

    pub fn visualize(&mut self, title: &str) {
        while !self.rl.window_should_close() {
            self.draw(&title);
        }
    }
}
