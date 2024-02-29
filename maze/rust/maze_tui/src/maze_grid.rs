use maze_lib::{algorithms::PathfindingState, Maze};
use ratatui::{
    layout::Alignment,
    style::{Style, Stylize},
    widgets::{block::{Position, Title}, Block, Borders, Widget},
};

pub struct MazeGrid {
    maze: Maze,
    state: PathfindingState,
}

impl MazeGrid {
    pub fn new(maze: &Maze, state: PathfindingState) -> Self {
        MazeGrid {
            maze: maze.clone(),
            state,
        }
    }
}

impl Widget for MazeGrid {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let state_title = Title::from(match self.state{
            PathfindingState::NotStarted => "Pathfinding: Not started",
            PathfindingState::Running => "Pathfinding: Running",
            PathfindingState::Finished => "Pathfinding: Finished",
        });
        let maze_block_title = Title::from("Maze crawler".bold());
        Block::default()
            .title(maze_block_title.alignment(Alignment::Center))
            .title(state_title.position(Position::Bottom))
            .borders(Borders::ALL)
            .render(area, buf);

        let rows = self.maze.width;
        let cols = self.maze.height;

        for col in 0..cols {
            for row in 0..rows {
                let (value, color) = match self.maze.get_cell(col, row) {
                    maze_lib::MazeCell::Wall => ("██", Style::default().on_black().white()),
                    maze_lib::MazeCell::Path => ("  ", Style::default().on_black()),
                    maze_lib::MazeCell::Entrance => ("░░", Style::default().blue()),
                    maze_lib::MazeCell::Exit => ("╒╕", Style::default().red()),
                    maze_lib::MazeCell::Visited => ("  ", Style::default().on_dark_gray()),
                    maze_lib::MazeCell::FinalPath => ("  ", Style::default().on_light_green()),
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
