use maze_lib::{
    algorithms::{PathfindingState, Point},
    CellType, Maze,
};
use ratatui::{buffer::Buffer, prelude::Rect};
use ratatui::{
    style::{Style, Stylize},
    widgets::{block::Title, Block, Borders, Widget},
};

use crate::animation::AnimationState;

pub struct MazeGrid {
    maze: Maze,
    state: PathfindingState,
    animation_state: AnimationState,
}

impl MazeGrid {
    pub fn new(maze: &Maze, state: PathfindingState, animation_state: AnimationState) -> Self {
        MazeGrid {
            maze: maze.clone(),
            state,
            animation_state,
        }
    }
}

impl Widget for MazeGrid {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let state_title = Title::from(match self.state {
            PathfindingState::NotStarted => "Pathfinding: Not started",
            PathfindingState::Running => "Pathfinding: Running",
            PathfindingState::Finished => "Pathfinding: Finished",
        });
        let animation_state_title = Title::from(match self.animation_state {
            AnimationState::Running => "Animation: Running",
            AnimationState::Paused => "Animation: Paused",
            AnimationState::NotRunning => "Animation: Not running",
        });
        let maze_block_title = Title::from("Maze crawler".bold());
        Block::default()
            .title(maze_block_title)
            .title(state_title)
            .title(animation_state_title)
            .borders(Borders::ALL)
            .render(area, buf);

        let rows = self.maze.width;
        let cols = self.maze.height;

        for col in 0..cols {
            for row in 0..rows {
                let (value, color) = match self.maze.get_cell(Point { x: col, y: row }).get_type() {
                    CellType::Wall => ("██", Style::default().on_black().white()),
                    CellType::Path => ("  ", Style::default().on_black()),
                    CellType::Entrance => ("░░", Style::default().blue()),
                    CellType::Exit => ("╒╕", Style::default().red()),
                    CellType::Visited => ("  ", Style::default().on_light_yellow()),
                    CellType::FinalPath => ("  ", Style::default().on_light_green()),
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
