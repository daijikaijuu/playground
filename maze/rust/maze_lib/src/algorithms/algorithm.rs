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
}

impl Algorithm {
    pub const ALL: [Algorithm; 6] = [
        Algorithm::AStar,
        Algorithm::Backtracking,
        Algorithm::BellmanFord,
        Algorithm::BFS,
        Algorithm::DFS,
        Algorithm::Dijkstra,
    ];
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
            }
        )
    }
}

pub const MOVEMENTS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
