use crate::{Maze, MazeType};

use super::Point;

pub trait MazeGenerationAlgorithm {
    fn generate(
        &mut self,
        maze_type: MazeType,
        width: usize,
        height: usize,
        entrance: Point,
    ) -> Option<Maze>;
}
