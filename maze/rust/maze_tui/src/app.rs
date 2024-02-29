use std::{
    collections::VecDeque,
    error,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use enum_iterator::{next_cycle, previous_cycle};
use maze_lib::{
    algorithms::{self, Algorithm, BellmanFord, PathfindingAlgorithm, PathfindingResult},
    Maze, MazeGenerator,
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::maze_grid::MazeGrid;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Default)]
pub struct App {
    maze: Maze,
    selected_algorithm: Algorithm,
    animation_steps: VecDeque<Maze>,
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        let mut maze = Maze::new(41, 41);
        maze.generate_maze(1, 1);
        App {
            maze,
            selected_algorithm: Algorithm::default(),
            animation_steps: VecDeque::new(),
            running: true,
        }
    }

    pub fn exit(&mut self) {
        self.running = false;
    }

    pub fn reset_maze(&mut self) {
        self.animation_steps.clear();
        self.maze = Maze::new(41, 41);
        self.maze.generate_maze(1, 1);
    }

    pub fn find_path(&mut self) {
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
            Algorithm::BellmanFord => {
                let mut bellman_ford = BellmanFord::default();
                bellman_ford.find_path(&mut maze, &sender);
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

    pub fn tick(&mut self) {
        if !self.animation_steps.is_empty() {
            self.maze = self.animation_steps.pop_front().unwrap();
        }
    }

    pub fn select_next_algorithm(&mut self) {
        self.selected_algorithm = next_cycle(&self.selected_algorithm);
    }

    pub fn select_previous_algorithm(&mut self) {
        self.selected_algorithm = previous_cycle(&self.selected_algorithm);
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
