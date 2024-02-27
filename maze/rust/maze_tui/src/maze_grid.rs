use maze_lib::Maze;
use ratatui::{style::{Style, Stylize}, widgets::Widget};

pub struct MazeGrid {
    maze: Maze,
}

impl MazeGrid {
    pub fn new(maze: &Maze) -> Self {
        MazeGrid { maze: maze.clone() }
    }
}

impl Widget for MazeGrid {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let rows = self.maze.width;
        let cols = self.maze.height;

        for col in 0..cols {
            for row in 0..rows {
                let (value, color) = match self.maze.get_cell(col, row) {
                    maze_lib::MazeCell::Wall => ("██", Style::default().on_black().white()),
                    maze_lib::MazeCell::Path => ("  ", Style::default().on_black()),
                    maze_lib::MazeCell::Entrance => ("░░", Style::default().blue()),
                    maze_lib::MazeCell::Exit => ("╒╕", Style::default().red()),
                    maze_lib::MazeCell::Visited => ("  ", Style::default()),
                    maze_lib::MazeCell::FinalPath => ("  ", Style::default()),
                };
                buf.set_string(
                    area.left() + 1 + (col * 2) as u16,
                    area.top() + 1 + row as u16,
                    value,
                    color,
                );
            }
        }
    }
}
