use crate::visualization::MazeVisualization;

pub trait PathfindingAlgorithm {
    fn find_path(&mut self, visualizer: &mut MazeVisualization) -> bool;
    fn name(&self) -> &str;
}
