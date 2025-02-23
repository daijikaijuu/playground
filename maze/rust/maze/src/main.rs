use iced::{
    time,
    widget::{button, column, pick_list, row, text, vertical_space},
    Element, Subscription, Theme,
};
mod ui;

use maze_lib::algorithms::Algorithm;
use ui::MazeGrid;

#[derive(Debug)]
struct MainWindow {
    maze_grid: MazeGrid,
    selected_algorithm: Option<Algorithm>,
    selected_generator: Option<Algorithm>,
}

#[derive(Debug, Clone)]
enum Message {
    AlgorithmSelected(Algorithm),
    GeneratorSelected(Algorithm),
    FindPath,
    MazeGrid(ui::maze_grid::Message),
    Tick,
}

impl MainWindow {
    fn update(&mut self, message: Message) {
        match message {
            Message::AlgorithmSelected(algorithm) => {
                self.selected_algorithm = Some(algorithm);
                self.maze_grid.selected_algorithm = algorithm;
            }
            Message::GeneratorSelected(algorithm) => {
                self.selected_generator = Some(algorithm);
                self.maze_grid.selected_generator = algorithm;
            }
            Message::MazeGrid(message) => {
                self.maze_grid.update(message);
            }
            Message::FindPath => self.maze_grid.start(),
            Message::Tick => {
                self.maze_grid.tick();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let algorithm_selector_list = pick_list(
            Algorithm::pathfinding_algorithms(),
            self.selected_algorithm,
            Message::AlgorithmSelected,
        )
        .placeholder("Choose a pathfinding algorithm");

        let generator_selector_list = pick_list(
            Algorithm::maze_generation_algorithms(),
            self.selected_generator,
            Message::GeneratorSelected,
        )
        .placeholder("Choose a maze generator");

        let button_controls = row![
            button("Generate maze")
                .on_press(Message::MazeGrid(ui::maze_grid::Message::GenerateMaze)),
            button("Find path").on_press(Message::FindPath),
        ]
        .spacing(10);

        let top_controls = row![
            text("Maze crawler").size(20),
            algorithm_selector_list,
            generator_selector_list,
            button_controls,
        ]
        .spacing(10);

        let top_space = vertical_space().height(5);
        let bottom_space = vertical_space().height(5);
        let maze_view = self.maze_grid.view().map(Message::MazeGrid);
        let content = column![top_space, top_controls, bottom_space, maze_view];

        content.into()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(std::time::Duration::from_millis(20)).map(|_| Message::Tick)
    }
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            maze_grid: MazeGrid::new(),
            selected_algorithm: Some(Algorithm::default()),
            selected_generator: Some(Algorithm::DFS),
        }
    }
}

fn main() -> iced::Result {
    iced::application("Maze crawler", MainWindow::update, MainWindow::view)
        .theme(|_| Theme::Dark)
        .subscription(MainWindow::subscription)
        .antialiasing(true)
        .centered()
        .run()
}
