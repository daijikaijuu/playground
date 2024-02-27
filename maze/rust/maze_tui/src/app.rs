use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use maze_lib::{Maze, MazeGenerator};
use ratatui::{
    layout::Alignment,
    style::Stylize,
    widgets::{block::Title, Block, Borders, Paragraph, Widget},
    Frame,
};

use crate::{maze_grid::MazeGrid, tui};

#[derive(Debug, Default)]
pub struct App {
    maze: Maze,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        let mut maze = Maze::new(41, 41);
        maze.generate_maze(1, 1);
        App {
            maze,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: event::KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let title = Title::from("Maze crawler".bold());
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .borders(Borders::ALL).render(area, buf);
        MazeGrid::new(&self.maze).render(area, buf);
    }
}
