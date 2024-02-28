use std::{
    collections::VecDeque,
    io,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use enum_iterator::{next_cycle, previous_cycle};
use maze_lib::{
    algorithms::{self, Algorithm, PathfindingAlgorithm, PathfindingResult},
    Maze, MazeGenerator,
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
    Frame,
};

use crate::{maze_grid::MazeGrid, tui};

#[derive(Debug, Default)]
pub struct App {
    maze: Maze,
    selected_algorithm: Algorithm,
    animation_steps: VecDeque<Maze>,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        let mut maze = Maze::new(41, 41);
        maze.generate_maze(1, 1);
        App {
            maze,
            selected_algorithm: Algorithm::default(),
            animation_steps: VecDeque::new(),
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(10);

        while !self.exit {
            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                println!("Ticked");
                last_tick = Instant::now();
            }

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
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => self.exit(),

            KeyCode::Up => self.selected_algorithm = previous_cycle(&self.selected_algorithm),
            KeyCode::Down => self.selected_algorithm = next_cycle(&self.selected_algorithm),
            KeyCode::Enter => self.find_path(),
            KeyCode::Char('c') | KeyCode::Char('C') => self.reset_maze(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn reset_maze(&mut self) {
        self.animation_steps.clear();
        self.maze = Maze::new(41, 41);
        self.maze.generate_maze(1, 1);
    }

    fn find_path(&mut self) {
        self.animation_steps.clear();
        self.maze.reset();

        let (sender, receiver): (Sender<PathfindingResult>, Receiver<PathfindingResult>) =
            mpsc::channel();

        let selected_algorithm = self.selected_algorithm;
        let mut maze = self.maze.clone();
        let handle = thread::spawn(move || match selected_algorithm {
            Algorithm::AStar => {
                let mut astar = algorithms::AStar::new();
                astar.find_path(&mut maze, &sender)
            }
            Algorithm::Backtracking => {
                let mut backtracking = algorithms::Backtracking::new();
                backtracking.find_path(&mut maze, &sender);
            }
            Algorithm::BFS => {
                let mut bfs = algorithms::BFS::new();
                bfs.find_path(&mut maze, &sender);
            }
            Algorithm::DFS => {
                let mut dfs = algorithms::DFS::new();
                dfs.find_path(&mut maze, &sender);
            }
            Algorithm::Dijkstra => {
                let mut dijktra = algorithms::Dijkstra::new();
                dijktra.find_path(&mut maze, &sender);
            }
        });

        while let Ok(recieved_result) = receiver.recv() {
            self.animation_steps.push_back(recieved_result.maze);
        }

        handle.join().expect("Failed to join thread");
    }

    fn on_tick(&mut self) {
        println!("{:?}", self.animation_steps.len());
        if !self.animation_steps.is_empty() {
            self.maze = self.animation_steps.pop_front().unwrap();
        }
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(area);
        MazeGrid::new(&self.maze).render(layout[0], buf);

        let algs = Algorithm::ALL
            .map(|alg| {
                if alg == self.selected_algorithm {
                    Line::styled(alg.to_string(), Style::new().fg(Color::Green))
                } else {
                    Line::from(alg.to_string())
                }
            })
            .to_vec();
        Paragraph::new(algs)
            .block(Block::default().title("Algorithms").borders(Borders::ALL))
            .render(layout[1], buf);
    }
}
