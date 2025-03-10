use iced::{
    time,
    widget::{button, column, pick_list, row, text, vertical_space},
    Element, Subscription, Theme,
};
mod ui;

use maze_lib::{algorithms::Algorithm, MazeType};
use ui::MazeGrid;

#[derive(Debug)]
struct MainWindow {
    maze_grid: MazeGrid,
    selected_maze_type: Option<MazeType>,
    selected_algorithm: Option<Algorithm>,
    selected_generator: Option<Algorithm>,
}

#[derive(Debug, Clone)]
enum Message {
    MazeTypeSelected(MazeType),
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
            Message::MazeTypeSelected(maze_type) => {
                self.selected_maze_type = Some(maze_type);
                self.maze_grid.selected_maze_type = maze_type;
                if let Some(selected_algorithm) = self.selected_generator {
                    let maze_type = self.selected_maze_type.unwrap_or_default();
                    if !Algorithm::maze_generation_algorithms(maze_type)
                        .contains(&selected_algorithm)
                    {
                        self.selected_generator = Algorithm::maze_generation_algorithms(maze_type)
                            .first()
                            .copied();
                    }
                }
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
        let maze_type_selector_list = pick_list(
            MazeType::cell_types(),
            self.selected_maze_type,
            Message::MazeTypeSelected,
        )
        .placeholder("Choose a maze type");

        let algorithm_selector_list = pick_list(
            Algorithm::pathfinding_algorithms(),
            self.selected_algorithm,
            Message::AlgorithmSelected,
        )
        .placeholder("Choose a pathfinding algorithm");

        let generator_selector_list = pick_list(
            Algorithm::maze_generation_algorithms(self.selected_maze_type.unwrap_or_default()),
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

        let left_controls = column![
            text("Maze crawler").size(20),
            maze_type_selector_list,
            algorithm_selector_list,
            generator_selector_list,
            button_controls,
        ]
        .spacing(10);

        let left_space = vertical_space().width(5);
        let right_space = vertical_space().width(5);
        let maze_view = self.maze_grid.view().map(Message::MazeGrid);
        let content = row![left_space, left_controls, right_space, maze_view];

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
            selected_maze_type: Some(MazeType::default()),
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
