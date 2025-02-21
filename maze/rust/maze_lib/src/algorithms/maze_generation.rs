use crate::Maze;

pub trait MazeGenerationAlgorithm {
    fn generate(
        &mut self,
        width: usize,
        height: usize,
        start_x: usize,
        start_y: usize,
    ) -> Option<Maze>;
}
