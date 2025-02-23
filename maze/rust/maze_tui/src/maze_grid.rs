use maze_lib::{
    algorithms::{PathfindingAnimationState, PathfindingState, Point},
    Maze, MazeCell, SlimWallsCellType, ThickMazeCellType,
};
use ratatui::{
    style::{Style, Stylize},
    widgets::{block::Title, Block, Borders, Widget},
};

pub struct MazeGrid {
    maze: Maze,
    state: PathfindingState,
    animation_state: PathfindingAnimationState,
}

impl MazeGrid {
    pub fn new(
        maze: &Maze,
        state: PathfindingState,
        animation_state: PathfindingAnimationState,
    ) -> Self {
        MazeGrid {
            maze: maze.clone(),
            state,
            animation_state,
        }
    }
}

impl Widget for MazeGrid {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let state_title = Title::from(match self.state {
            PathfindingState::NotStarted => "Pathfinding: Not started",
            PathfindingState::Running => "Pathfinding: Running",
            PathfindingState::Finished => "Pathfinding: Finished",
        });
        let animation_state_title = Title::from(match self.animation_state {
            PathfindingAnimationState::Running => "Animation: Running",
            PathfindingAnimationState::Paused => "Animation: Paused",
            PathfindingAnimationState::NotRunning => "Animation: Not running",
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
                let (value, color) = match self.maze.get_cell(Point { x: col, y: row }) {
                    MazeCell::Thick(thick_cell) => match thick_cell.cell {
                        ThickMazeCellType::Wall => ("██", Style::default().on_black().white()),
                        ThickMazeCellType::Path => ("  ", Style::default().on_black()),
                        ThickMazeCellType::Entrance => ("░░", Style::default().blue()),
                        ThickMazeCellType::Exit => ("╒╕", Style::default().red()),
                        ThickMazeCellType::Visited => ("  ", Style::default().on_light_yellow()),
                        ThickMazeCellType::FinalPath => ("  ", Style::default().on_light_green()),
                    },
                    MazeCell::Slim(slim_cell) => match slim_cell.cell {
                        SlimWallsCellType::Path => ("  ", Style::default().on_black()),
                        SlimWallsCellType::Entrance => ("░░", Style::default().blue()),
                        SlimWallsCellType::Exit => ("╒╕", Style::default().red()),
                        SlimWallsCellType::Visited => ("  ", Style::default().on_light_yellow()),
                        SlimWallsCellType::FinalPath => ("  ", Style::default().on_light_green()),
                    },
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
