use std::time::Duration;

use iced::{
    executor, theme, time,
    widget::{button, column, pick_list, row, text, vertical_space},
    window, Application, Command, Settings, Theme,
};
use ui::MazeGrid;

mod algorithms;
mod maze;
mod ui;

#[derive(Debug)]
struct MainWindow {
    maze_grid: MazeGrid,
    selected_algorithm: Option<Algorithm>,
}

#[derive(Debug, Clone)]
enum Message {
    AlgorithmSelected(Algorithm),
    FindPath,
    MazeGrid(ui::maze_grid::Message),
    Tick,
}

impl Application for MainWindow {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            MainWindow {
                maze_grid: MazeGrid::new(),
                selected_algorithm: Some(Algorithm::default()),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Maze crawler")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::AlgorithmSelected(algorithm) => {
                self.selected_algorithm = Some(algorithm);
                self.maze_grid.selected_algorithm = algorithm;
            }
            Message::MazeGrid(message) => {
                self.maze_grid.update(message);
            }
            Message::FindPath => self.maze_grid.start(),
            Message::Tick => {
                println!("Tok");
                return Command::perform(async {}, |_| {
                    Message::MazeGrid(ui::maze_grid::Message::Tick)
                });
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let algorithm_selector_list = pick_list(
            &Algorithm::ALL[..],
            self.selected_algorithm,
            Message::AlgorithmSelected,
        )
        .placeholder("Choose an algorithm");

        let button_controls = row![
            button("Generate maze")
                .on_press(Message::MazeGrid(ui::maze_grid::Message::GenerateMaze))
                .style(theme::Button::Secondary),
            button("Find path").on_press(Message::FindPath),
            //button("Animate"), //.on_press(Message::MazeGrid(ui::maze_grid::Message::Animate)),
        ]
        .spacing(10);

        let top_controls = row![
            text(format!("Maze crawler")).size(20),
            algorithm_selector_list,
            button_controls,
        ]
        .spacing(10);

        column![
            vertical_space().height(5),
            top_controls,
            vertical_space().height(5),
            self.maze_grid
                .view()
                .map(move |message| Message::MazeGrid(message))
        ]
        .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        time::every(Duration::from_millis(20)).map(|_| Message::Tick)
    }
}

fn main() -> iced::Result {
    MainWindow::run(Settings {
        antialiasing: true,
        window: window::Settings {
            position: window::Position::Centered,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
// // Backtracking algorithm
// let mut maze_visualization =
//     MazeVisualization::new(ROWS, COLS, &mut self.rl, &self.thread);
// let mut backtracking_algorithm = Backtracking::new();
// if backtracking_algorithm.find_path(&mut maze_visualization) {}

// thread::sleep(Duration::from_secs(2));

// // AStar algorithm
// let mut maze_visualization =
//     MazeVisualization::new(ROWS, COLS, &mut self.rl, &self.thread);
// let mut astar_algorithm = AStar::new(20);
// if astar_algorithm.find_path(&mut maze_visualization) {}

// maze_visualization.visualize(astar_algorithm.name());

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Algorithm {
    #[default]
    Backtracking,
    AStar,
}

impl Algorithm {
    const ALL: [Algorithm; 2] = [Algorithm::Backtracking, Algorithm::AStar];
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Algorithm::Backtracking => "Backtracking",
                Algorithm::AStar => "AStar",
            }
        )
    }
}
