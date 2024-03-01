use std::{
    cmp::{max, min},
    collections::VecDeque,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use iced::{
    alignment, mouse, theme,
    widget::{
        canvas::{self, Cache, Geometry, Path, Stroke},
        column, container, row, text, Canvas,
    },
    Color, Element, Length, Point, Rectangle, Renderer, Size, Theme,
};

use maze_lib::{algorithms::*, Maze, MazeCell, MazeGenerator};

#[derive(Debug)]
pub struct MazeGrid {
    maze: Maze,
    grid_cache: Cache,
    animation_queue: VecDeque<Maze>,
    pub selected_algorithm: Algorithm,
    pathfinding_stats: Option<PathfindingStats>,
}

#[derive(Debug, Clone)]
pub enum Message {
    GenerateMaze,
    SelectAlgorithm(Algorithm),
    Tick,
}

impl MazeGrid {
    pub fn new() -> Self {
        MazeGrid {
            maze: Maze::new(41, 41),
            grid_cache: Cache::default(),
            selected_algorithm: Algorithm::default(),
            animation_queue: VecDeque::new(),
            pathfinding_stats: None,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let canvas = Canvas::new(self).width(Length::FillPortion(4)).height(Length::Fill);

        // Stats
        if let Some(st) = self.pathfinding_stats {
            let steps = text(format!("Steps: {}", st.steps))
                .horizontal_alignment(alignment::Horizontal::Left);
            let stats = column!(steps).width(Length::Shrink).padding(5);
            let stats_container = container(stats)
                .width(Length::FillPortion(1))
                .style(theme::Container::Box);
            row![canvas, stats_container].align_items(iced::Alignment::Start).into()
        } else {
            canvas.into()
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::SelectAlgorithm(algorithm) => {
                self.selected_algorithm = algorithm;
                self.grid_cache.clear();
                self.animation_queue.clear();
            }
            Message::GenerateMaze => self.generate_maze(),
            Message::Tick => {
                self.tick();
            }
        }
    }

    pub fn tick(&mut self) {
        if !self.animation_queue.is_empty() {
            self.maze = self
                .animation_queue
                .pop_front()
                .expect("Cannot pop animation frame");
            self.grid_cache.clear();
        }
    }

    pub fn start(&mut self) {
        // Reset maze
        self.pathfinding_stats = None;
        self.grid_cache.clear();
        self.animation_queue.clear();
        self.maze = self.maze.from_original();

        let (sender, reciever): (Sender<PathfindingResult>, Receiver<PathfindingResult>) =
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
                let mut bellman_ford = BellmanFord::default();
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
        });

        while let Ok(recieved_result) = reciever.recv() {
            self.maze = recieved_result.maze.clone();
            self.animation_queue.push_back(recieved_result.maze);
            self.pathfinding_stats = recieved_result.stats;
        }

        handle.join().expect("Failed to join thread");
        self.grid_cache.clear();
    }

    fn generate_maze(&mut self) {
        self.maze.generate_maze(1, 1);
        self.grid_cache.clear();
        self.animation_queue.clear();
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
                    frame.fill_rectangle(
                        starting_point,
                        size,
                        match self.maze.get_cell(row, col) {
                            MazeCell::Wall => Color::from_rgb8(100, 100, 100),
                            MazeCell::Path => Color::from_rgb8(255, 255, 255),
                            MazeCell::Entrance => Color::from_rgb8(0, 0, 255),
                            MazeCell::Exit => Color::from_rgb8(255, 0, 0),
                            MazeCell::Visited => Color::from_rgb8(0, 0, 100),
                            MazeCell::FinalPath => Color::from_rgb8(100, 155, 255),
                        },
                    );
                    frame.stroke(
                        &Path::rectangle(starting_point, size),
                        Stroke::default()
                            .with_width(1.0)
                            .with_color(Color::from_rgb8(55, 55, 55)),
                    )
                }
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
