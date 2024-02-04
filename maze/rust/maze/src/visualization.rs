use crate::maze::{Maze, MazeCell, MazeGenerator};
use raylib::prelude::*;

pub struct MazeVisualization<'a> {
    pub maze: Maze,
    pub cell_size: i32,
    rl: &'a mut RaylibHandle,
    thread: &'a RaylibThread,
}

impl<'a> MazeVisualization<'a> {
    pub fn new(
        width: usize,
        height: usize,
        cell_size: i32,
        rl: &'a mut RaylibHandle,
        thread: &'a RaylibThread,
    ) -> Self {
        let mut maze = Maze::new(width, height);
        maze.generate_maze(1, 1);
        println!("{:?}", maze);
        MazeVisualization {
            maze,
            cell_size,
            rl,
            thread,
        }
    }

    pub fn draw(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::BLACK);

        for y in 0..self.maze.height {
            for x in 0..self.maze.width {
                let color = match self.maze.get_cell(x, y) {
                    MazeCell::Wall => Color::LIGHTGRAY,
                    MazeCell::Path => Color::BLACK,
                    MazeCell::Entrance => Color::BLUE,
                    MazeCell::Exit => Color::RED,
                    MazeCell::Visited => Color::GREEN,
                    MazeCell::FinalPath => Color::GOLD,
                };

                d.draw_rectangle(
                    x as i32 * self.cell_size,
                    y as i32 * self.cell_size,
                    self.cell_size,
                    self.cell_size,
                    color,
                );
            }
        }
    }

    pub fn visualize(&mut self) {
        while !self.rl.window_should_close() {
            self.draw();
        }
    }
}
