use std::sync::mpsc::Sender;

use crate::maze::Maze;

pub trait PathfindingAlgorithm {
    fn find_path(&mut self, maze: &mut Maze, sender: &Sender<Maze>);
    fn name(&self) -> &str;
}
