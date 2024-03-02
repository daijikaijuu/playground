use std::sync::mpsc::Sender;

use crate::maze::Maze;

use super::Algorithm;

#[derive(Default, Debug, Clone, Copy)]
pub struct PathfindingStats {
    pub steps: usize,
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub enum PathfindingState {
    #[default]
    NotStarted,
    Running,
    Finished,
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub enum PathfindingAnimationState {
    #[default]
    NotRunning,
    Running,
    Paused,
}

pub struct PathfindingResult {
    pub stats: Option<PathfindingStats>,
    pub maze: Maze,
}

pub trait PathfindingAlgorithm {
    fn find_path(&mut self, maze: &mut Maze, sender: &Sender<PathfindingResult>);

    fn get_stats(&self) -> Option<PathfindingStats>;

    fn name(&self) -> Algorithm;
}

impl PathfindingStats {
    pub fn new_step(&mut self) {
        self.steps += 1;
    }
}
