use std::sync::mpsc::Sender;

use crate::{Maze, MazeType};

use super::{PathfindingResult, Point};

pub trait MazeGenerationAlgorithm {
    fn generate(
        &mut self,
        maze_type: MazeType,
        width: usize,
        height: usize,
        entrance: Point,
        sender: Option<&Sender<PathfindingResult>>,
    ) -> Option<Maze>;
}
