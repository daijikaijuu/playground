pub mod algorithm_selector;
pub mod main_window;

use std::{ffi::CString, thread, time::Duration};

use raylib::{prelude::*, RaylibHandle, RaylibThread};

use crate::{
    astar::AStar, backtracking::Backtracking, pathfinding::PathfindingAlgorithm,
    visualization::MazeVisualization,
};

use self::{
    algorithm_selector::AlgorithmSelector,
    main_window::{MainWindow, ScreenState},
};

const ROWS: usize = 41;
const COLS: usize = 41;

pub struct Ui {
    pub rl: RaylibHandle,
    thread: RaylibThread,
    main_window: MainWindow,
}

impl Ui {
    pub fn new() -> Self {
        let (rl, thread) = raylib::init().size(800, 800).title("Maze crawler").build();

        Ui {
            rl,
            thread,
            main_window: MainWindow::new(),
        }
    }

    pub fn run(&mut self) {
        let mut alg_selector = AlgorithmSelector::new();

        while !self.rl.window_should_close() {
            match self.main_window.screen_state {
                ScreenState::Menu => {
                    let mut d = self.rl.begin_drawing(&self.thread);
                    d.clear_background(Color::BLACK);

                    // Create UI
                    d.gui_enable();

                    let title = "Maze crawler";
                    let font_size = d.gui_get_font().baseSize;
                    let title_width = raylib::text::measure_text(title, font_size);
                    let _title_label = d.gui_label(
                        Rectangle::new(
                            (d.get_screen_width() / 2) as f32 - (title_width / 2) as f32,
                            0.0,
                            0.0,
                            30.0,
                        ),
                        Some(CString::new(title).unwrap().as_c_str()),
                    );

                    let mut active: i32 = 0;
                    let algorithm_selector = d.gui_dropdown_box(
                        Rectangle::new(10.0, 30.0, 100.0, 40.0),
                        Some(
                            CString::new(alg_selector.selected_name())
                                .unwrap()
                                .as_c_str(),
                        ),
                        &mut active,
                        false,
                    );
                }
                ScreenState::Pathfinding => {
                    // Backtracking algorithm
                    let mut maze_visualization =
                        MazeVisualization::new(ROWS, COLS, &mut self.rl, &self.thread);
                    let mut backtracking_algorithm = Backtracking::new();
                    if backtracking_algorithm.find_path(&mut maze_visualization) {}

                    thread::sleep(Duration::from_secs(2));

                    // AStar algorithm
                    let mut maze_visualization =
                        MazeVisualization::new(ROWS, COLS, &mut self.rl, &self.thread);
                    let mut astar_algorithm = AStar::new(20);
                    if astar_algorithm.find_path(&mut maze_visualization) {}

                    maze_visualization.visualize(astar_algorithm.name());
                }
            }
        }
    }
}
