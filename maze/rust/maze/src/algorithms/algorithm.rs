#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Algorithm {
    AStar,
    #[default]
    Backtracking,
    BFS,
    Dijkstra,
}

impl Algorithm {
    pub const ALL: [Algorithm; 4] = [
        Algorithm::AStar,
        Algorithm::Backtracking,
        Algorithm::BFS,
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
                Algorithm::BFS => "Breadth-First Search (BFS)",
                Algorithm::Dijkstra => "Dijkstra's",
            }
        )
    }
}
