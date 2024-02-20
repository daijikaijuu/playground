use std::ffi::CString;

#[derive(Debug)]
pub enum PathfindingAlgorithms {
    Backtracking,
    AStar,
}

#[derive(Debug)]
pub struct AlgorithmSelector {
    pub selected: PathfindingAlgorithms,
}

impl AlgorithmSelector {
    pub fn new() -> Self {
        AlgorithmSelector {
            selected: PathfindingAlgorithms::Backtracking,
        }
    }

    pub fn get_index(&self) -> u8 {
        match self.selected {
            PathfindingAlgorithms::Backtracking => 0,
            PathfindingAlgorithms::AStar => 1,
        }
    }

    pub fn set_index(&mut self, index: u8) {
        match index {
            0 => self.selected = PathfindingAlgorithms::Backtracking,
            1 => self.selected = PathfindingAlgorithms::AStar,
            _ => panic!("Not implemented"),
        }
    }

    pub fn selected_name(&self) -> &str {
        match self.selected {
            PathfindingAlgorithms::Backtracking => "Backtracking",
            PathfindingAlgorithms::AStar => "AStar",
        }
    }
}
