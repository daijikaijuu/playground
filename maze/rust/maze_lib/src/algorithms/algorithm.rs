use super::MazeGenerationAlgorithm;
use super::{Backtracking, DFS, WFC};
use enum_iterator::Sequence;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Sequence)]
pub enum Algorithm {
    AStar,
    #[default]
    Backtracking,
    BellmanFord,
    BFS,
    DFS,
    Dijkstra,
    WFC,
}

impl Algorithm {
    pub const ALL: [Algorithm; 7] = [
        Algorithm::AStar,
        Algorithm::Backtracking,
        Algorithm::BellmanFord,
        Algorithm::BFS,
        Algorithm::DFS,
        Algorithm::Dijkstra,
        Algorithm::WFC,
    ];

    pub fn maze_generation_algorithms() -> Vec<Algorithm> {
        vec![Algorithm::DFS, Algorithm::Backtracking, Algorithm::WFC]
    }

    pub fn pathfinding_algorithms() -> Vec<Algorithm> {
        vec![
            Algorithm::AStar,
            Algorithm::Backtracking,
            Algorithm::BellmanFord,
            Algorithm::BFS,
            Algorithm::DFS,
            Algorithm::Dijkstra,
        ]
    }

    pub fn is_pathfinding_algorithm(&self) -> bool {
        Self::pathfinding_algorithms().contains(self)
    }

    pub fn get_maze_generator(&self) -> Option<Box<dyn MazeGenerationAlgorithm>> {
        match self {
            Algorithm::DFS => Some(Box::new(DFS::default())),
            Algorithm::Backtracking => Some(Box::new(Backtracking::default())),
            Algorithm::WFC => Some(Box::new(WFC::default())),
            _ => None,
        }
    }
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Algorithm::AStar => "AStar",
                Algorithm::Backtracking => "Backtracking",
                Algorithm::BellmanFord => "Bellman-Ford",
                Algorithm::BFS => "Breadth-First Search (BFS)",
                Algorithm::DFS => "Depth-First Search (DFS)",
                Algorithm::Dijkstra => "Dijkstra's",
                Algorithm::WFC => "Wave Function Collapse",
            }
        )
    }
}

pub const MOVEMENTS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
pub const MOVEMENTS_X2: [(i32, i32); 4] = [(0, 2), (2, 0), (0, -2), (-2, 0)];
