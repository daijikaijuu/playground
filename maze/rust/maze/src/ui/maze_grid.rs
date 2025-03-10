use std::{
    cmp::{max, min},
    collections::VecDeque,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use iced::{
    alignment, mouse,
    widget::{
        canvas::{self, Cache, Geometry, Path, Stroke},
        column, container, row, text, Canvas,
    },
    Color, Element, Length, Point, Rectangle, Renderer, Size, Theme,
};

use maze_lib::{algorithms::*, CellType, Maze, MazeCell, MazeType, SlimWallsCellType};

use super::AnimationState;

#[derive(Debug)]
pub struct MazeGrid {
    maze: Maze,
    grid_cache: Cache,
    animation_queue: VecDeque<Maze>,
    animation_state: AnimationState,
    pub selected_algorithm: Algorithm,
    pub selected_generator: Algorithm,
    pub selected_maze_type: MazeType,
    pathfinding_stats: Option<PathfindingStats>,
    pathfinding_state: PathfindingState,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Message {
    GenerateMaze,
    SelectAlgorithm(Algorithm),
    SelectMazeType(MazeCell),
}

impl MazeGrid {
    pub fn new() -> Self {
        let selected_generator = Algorithm::DFS;
        let maze = selected_generator
            .get_maze_generator()
            .expect("Default generator should exist")
            .generate(
                maze_lib::MazeType::Thick,
                41,
                41,
                maze_lib::algorithms::Point { x: 1, y: 1 },
                None,
            )
            .unwrap();
        MazeGrid {
            maze,
            grid_cache: Cache::default(),
            selected_algorithm: selected_generator,
            selected_generator,
            animation_queue: VecDeque::new(),
            animation_state: AnimationState::default(),
            selected_maze_type: MazeType::Thick,
            pathfinding_stats: None,
            pathfinding_state: PathfindingState::default(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let canvas = Canvas::new(self)
            .width(Length::FillPortion(4))
            .height(Length::Fill);

        // Stats
        if let Some(st) = self.pathfinding_stats {
            let steps = text(format!("Steps: {}", st.steps)).align_x(alignment::Horizontal::Left);
            let stats = column!(steps).width(Length::Shrink).padding(5);
            let stats_container = container(stats).width(Length::FillPortion(1));
            row![canvas, stats_container]
                .align_y(iced::alignment::Vertical::Top)
                .into()
        } else {
            canvas.into()
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::SelectAlgorithm(algorithm) => {
                if algorithm.is_pathfinding_algorithm() {
                    self.selected_algorithm = algorithm;
                    self.grid_cache.clear();
                    self.animation_queue.clear();
                }
            }
            Message::GenerateMaze => self.generate_maze(),
            Message::SelectMazeType(_) => todo!(),
        }
    }

    pub fn tick(&mut self) {
        if !self.animation_queue.is_empty() {
            self.animation_state = AnimationState::Running;
            self.maze = self
                .animation_queue
                .pop_front()
                .expect("Cannot pop animation frame");
            self.grid_cache.clear();
        } else {
            self.animation_state = AnimationState::NotRunning;
        }
    }

    pub fn start(&mut self) {
        // Don't start pathfinding if the selected algorithm is not a pathfinding algorithm,
        // or animation queue is not empty
        if !self.selected_algorithm.is_pathfinding_algorithm() || !self.animation_queue.is_empty() {
            return;
        }

        // Reset maze
        self.pathfinding_stats = None;
        self.grid_cache.clear();
        self.animation_queue.clear();
        self.pathfinding_state = PathfindingState::Running;
        self.maze = self.maze.from_original();

        let (sender, receiver): (Sender<PathfindingResult>, Receiver<PathfindingResult>) =
            channel();

        let mut maze = self.maze.clone();
        let selected_algorithm = self.selected_algorithm;

        let handle = thread::spawn(move || match selected_algorithm {
            Algorithm::AStar => {
                let mut astar = AStar::new();
                astar.find_path(&mut maze, &sender);
            }
            Algorithm::Backtracking => {
                let mut backtracking = Backtracking::new();
                backtracking.find_path(&mut maze, &sender);
            }
            Algorithm::BellmanFord => {
                let mut bellman_ford = BellmanFord;
                bellman_ford.find_path(&mut maze, &sender);
            }
            Algorithm::BFS => {
                let mut bfs = BFS::new();
                bfs.find_path(&mut maze, &sender);
            }
            Algorithm::DFS => {
                let mut dfs = DFS::new();
                dfs.find_path(&mut maze, &sender);
            }
            Algorithm::Dijkstra => {
                let mut dijktra = Dijkstra::new();
                dijktra.find_path(&mut maze, &sender);
            }
            _ => unreachable!("Non-pathfinding algorithms should be filtered out"),
        });

        while let Ok(received_result) = receiver.recv() {
            self.maze = received_result.maze.clone();
            self.animation_queue.push_back(received_result.maze);
            self.pathfinding_stats = received_result.stats;
        }

        handle.join().expect("Failed to join thread");
        self.grid_cache.clear();

        self.pathfinding_state = PathfindingState::Finished;
    }

    fn generate_maze(&mut self) {
        self.grid_cache.clear();
        self.animation_queue.clear();
        self.pathfinding_state = PathfindingState::Running;
        self.maze = self.maze.from_original();
        let (sender, receiver): (Sender<PathfindingResult>, Receiver<PathfindingResult>) =
            channel();

        let maze_type = self.selected_maze_type;
        let selected_generator = self.selected_generator;
        let width = self.maze.width;
        let height = self.maze.height;
        let handle = thread::spawn(move || {
            if let Some(mut generator) = selected_generator.get_maze_generator() {
                generator.generate(
                    maze_type,
                    width,
                    height,
                    maze_lib::algorithms::Point::default(),
                    Some(&sender),
                );
            }
        });

        while let Ok(received_result) = receiver.recv() {
            self.animation_queue.push_back(received_result.maze.clone());
        }

        handle.join().expect("Failed to join thread");

        self.grid_cache.clear();
    }
}

impl canvas::Program<Message> for MazeGrid {
    type State = Interaction;

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let grid = self.grid_cache.draw(renderer, bounds.size(), |frame| {
            let rows = self.maze.width;
            let cols = self.maze.height;
            let min_bound = min(bounds.width as i32, bounds.height as i32);
            let max_size = max(rows, cols);
            let cell_size: f32 = min_bound as f32 / max_size as f32;

            for col in 0..cols {
                for row in 0..rows {
                    let starting_point = Point::new(col as f32 * cell_size, row as f32 * cell_size);
                    let size = Size::new(cell_size, cell_size);
                    let cell = self
                        .maze
                        .get_cell(maze_lib::algorithms::Point { x: row, y: col });
                    frame.fill_rectangle(
                        starting_point,
                        size,
                        match cell.get_type() {
                            CellType::Wall => Color::from_rgb8(100, 100, 100),
                            CellType::Path => Color::from_rgb8(255, 255, 255),
                            CellType::Entrance => Color::from_rgb8(0, 0, 255),
                            CellType::Exit => Color::from_rgb8(255, 0, 0),
                            CellType::Visited => Color::from_rgb8(0, 0, 100),
                            CellType::FinalPath => Color::from_rgb8(100, 155, 255),
                        },
                    );
                    match self.maze.maze_type {
                        MazeType::Thick => {
                            frame.stroke(
                                &Path::rectangle(starting_point, size),
                                Stroke::default()
                                    .with_width(1.0)
                                    .with_color(Color::from_rgb8(55, 55, 55)),
                            );
                        }
                        MazeType::Slim => {
                            if cell.has_right_wall() {
                                frame.stroke(
                                    &Path::line(
                                        Point::new(
                                            (col + 1) as f32 * cell_size,
                                            row as f32 * cell_size,
                                        ),
                                        Point::new(
                                            (col + 1) as f32 * cell_size,
                                            (row + 1) as f32 * cell_size,
                                        ),
                                    ),
                                    Stroke::default()
                                        .with_width(1.0)
                                        .with_color(Color::from_rgb8(0, 0, 0)),
                                );
                            }
                            if cell.has_left_wall() {
                                frame.stroke(
                                    &Path::line(
                                        Point::new(col as f32 * cell_size, row as f32 * cell_size),
                                        Point::new(
                                            col as f32 * cell_size,
                                            (row + 1) as f32 * cell_size,
                                        ),
                                    ),
                                    Stroke::default()
                                        .with_width(1.0)
                                        .with_color(Color::from_rgb8(0, 0, 0)),
                                );
                            }
                            if cell.has_top_wall() {
                                frame.stroke(
                                    &Path::line(
                                        Point::new(col as f32 * cell_size, row as f32 * cell_size),
                                        Point::new(
                                            (col + 1) as f32 * cell_size,
                                            row as f32 * cell_size,
                                        ),
                                    ),
                                    Stroke::default()
                                        .with_width(1.0)
                                        .with_color(Color::from_rgb8(0, 0, 0)),
                                );
                            }
                            if cell.has_bottom_wall() {
                                frame.stroke(
                                    &Path::line(
                                        Point::new(
                                            col as f32 * cell_size,
                                            (row + 1) as f32 * cell_size,
                                        ),
                                        Point::new(
                                            (col + 1) as f32 * cell_size,
                                            (row + 1) as f32 * cell_size,
                                        ),
                                    ),
                                    Stroke::default()
                                        .with_width(1.0)
                                        .with_color(Color::from_rgb8(0, 0, 0)),
                                );
                            }
                        }
                    }
                }
            }

            if self.animation_state == AnimationState::Running {
                frame.stroke(
                    &Path::rectangle(
                        Point::new(0.0, 0.0),
                        Size::new(cell_size * rows as f32, cell_size * cols as f32 - 1.0),
                    ),
                    Stroke::default()
                        .with_width(1.0)
                        .with_color(Color::from_rgb8(0, 255, 0)),
                );
            }

            if self.pathfinding_state == PathfindingState::Running {
                frame.stroke(
                    &Path::rectangle(
                        Point::new(0.0, 0.0),
                        Size::new(cell_size * rows as f32, cell_size * cols as f32 - 1.0),
                    ),
                    Stroke::default()
                        .with_width(1.0)
                        .with_color(Color::from_rgb8(255, 0, 0)),
                );
            }
        });

        vec![grid]
    }
}

pub enum Interaction {
    None,
}

impl Default for Interaction {
    fn default() -> Self {
        Self::None
    }
}
