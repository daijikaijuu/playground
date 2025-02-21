use crate::Maze;

pub trait MazeGenerationAlgorithm {
    fn generate(self, width: usize, height: usize, start_x: usize, start_y: usize) -> Option<Maze>;
}
