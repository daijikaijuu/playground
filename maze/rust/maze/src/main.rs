use iced::{
    executor,
    widget::{pick_list, row, text},
    window, Application, Command, Settings, Theme,
};

mod astar;
mod backtracking;
mod maze;
mod pathfinding;
mod visualization;

#[derive(Debug, Default)]
struct MainWindow {
    selected_algorithm: Option<Algorithm>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    AlgorithmSelected(Algorithm),
}

impl Application for MainWindow {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Maze crawler")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::AlgorithmSelected(algorithm) => {
                self.selected_algorithm = Some(algorithm);
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
        // let col = row![horizontal_space()];

        row![
            text(format!("Maze crawler")).size(20),
            algorithm_selector_list
        ]
        .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
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
