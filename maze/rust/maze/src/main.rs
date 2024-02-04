use crate::{
    pathfinding::{Backtracking, PathfindingAlgorithm},
    visualization::MazeVisualization,
};

mod maze;
mod pathfinding;
mod visualization;

const ROWS: usize = 41;
const COLS: usize = 41;

fn main() {
    let (mut rl, thread) = raylib::init().size(800, 800).title("Maze crawler").build();

    let mut maze_visualization = MazeVisualization::new(ROWS, COLS, 16, &mut rl, &thread);
    let mut backtracking_algorithm = Backtracking::new(20);
    backtracking_algorithm.find_path(&mut maze_visualization);

    maze_visualization.visualize();
}
