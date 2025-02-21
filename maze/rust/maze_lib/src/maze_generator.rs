pub trait MazeGenerator {
    fn generate_maze(&mut self, start_x: usize, start_y: usize);
}
