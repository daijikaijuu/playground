use std::{
    collections::VecDeque,
    error,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use enum_iterator::{next_cycle, previous_cycle};
use maze_lib::{
    algorithms::{
        self, Algorithm, BellmanFord, MazeGenerationAlgorithm, PathfindingAlgorithm,
        PathfindingResult, PathfindingState, Point, DFS,
    },
    Maze,
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{animation::AnimationState, maze_grid::MazeGrid};

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Default)]
pub struct App {
    maze: Maze,
    selected_algorithm: Algorithm,
    animation_steps: VecDeque<Maze>,
    pub running: bool,
    pathfinding_state: PathfindingState,
    animation_state: AnimationState,
}

impl App {
    pub fn new() -> Self {
        let mut dfs = DFS::new();
        let maze = dfs
            .generate(
                maze_lib::MazeType::Thick,
                41,
                41,
                Point { x: 1, y: 1 },
                None,
            )
            .unwrap();
        App {
            maze,
            selected_algorithm: Algorithm::default(),
            animation_steps: VecDeque::new(),
            running: true,
            pathfinding_state: PathfindingState::default(),
            animation_state: AnimationState::default(),
        }
    }

    pub fn exit(&mut self) {
        self.running = false;
    }

    pub fn reset_maze(&mut self) {
        self.animation_steps.clear();
        let mut dfs = DFS::new();
        self.maze = dfs
            .generate(
                maze_lib::MazeType::Thick,
                41,
                41,
                Point { x: 1, y: 1 },
                None,
            )
            .unwrap();
        self.animation_state = AnimationState::default();
        self.pathfinding_state = PathfindingState::default();
    }

    pub fn find_path(&mut self) {
        self.animation_steps.clear();
        self.maze.reset();
        self.pathfinding_state = PathfindingState::Running;

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
                let mut bellman_ford = BellmanFord;
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
            _ => {}
        });

        while let Ok(received_result) = receiver.recv() {
            self.animation_state = AnimationState::Running;
            self.animation_steps.push_back(received_result.maze);
        }

        handle.join().expect("Failed to join thread");
        self.pathfinding_state = PathfindingState::Finished;
    }

    pub fn tick(&mut self) {
        if self.animation_state == AnimationState::Running {
            if !self.animation_steps.is_empty() {
                self.maze = self.animation_steps.pop_front().unwrap();
            } else {
                self.animation_state = AnimationState::default();
            }
        }
    }

    pub fn select_next_algorithm(&mut self) {
        self.selected_algorithm = next_cycle(&self.selected_algorithm);
    }

    pub fn select_previous_algorithm(&mut self) {
        self.selected_algorithm = previous_cycle(&self.selected_algorithm);
    }

    pub fn pause_unpause_animation(&mut self) {
        match self.animation_state {
            AnimationState::NotRunning => {}
            AnimationState::Paused => self.animation_state = AnimationState::Running,
            AnimationState::Running => self.animation_state = AnimationState::Paused,
        };
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(area);
        MazeGrid::new(&self.maze, self.pathfinding_state, self.animation_state)
            .render(layout[0], buf);

        let algs = Algorithm::pathfinding_algorithms()
            .into_iter()
            .map(|alg| {
                if alg == self.selected_algorithm {
                    Line::styled(alg.to_string(), Style::new().fg(Color::Green))
                } else {
                    Line::from(alg.to_string())
                }
            })
            .collect::<Vec<Line>>();
        Paragraph::new(algs)
            .block(Block::default().title("Algorithms").borders(Borders::ALL))
            .render(layout[1], buf);
    }
}
