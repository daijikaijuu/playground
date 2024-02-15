use std::thread;
use std::time::Duration;

use astar::AStar;
use pathfinding::PathfindingAlgorithm;

use crate::backtracking::Backtracking;
use crate::visualization::MazeVisualization;

mod astar;
mod backtracking;
mod maze;
mod pathfinding;
mod visualization;

const ROWS: usize = 41;
const COLS: usize = 41;

fn main() {
    let (mut rl, thread) = raylib::init().size(800, 800).title("Maze crawler").build();

    // Backtracking algorithm
    let mut maze_visualization = MazeVisualization::new(ROWS, COLS, &mut rl, &thread);
    let mut backtracking_algorithm = Backtracking::new();
    if backtracking_algorithm.find_path(&mut maze_visualization) {}

    thread::sleep(Duration::from_secs(2));

    // AStar algorithm
    let mut maze_visualization = MazeVisualization::new(ROWS, COLS, &mut rl, &thread);
    let mut astar_algorithm = AStar::new(20);
    if astar_algorithm.find_path(&mut maze_visualization) {}

    maze_visualization.visualize(astar_algorithm.name());
}
