#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Algorithm {
    #[default]
    Backtracking,
    AStar,
    Dijkstra,
}

impl Algorithm {
    pub const ALL: [Algorithm; 3] = [
        Algorithm::Backtracking,
        Algorithm::AStar,
        Algorithm::Dijkstra,
    ];
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Algorithm::Backtracking => "Backtracking",
                Algorithm::AStar => "AStar",
                Algorithm::Dijkstra => "Dijkstra's",
            }
        )
    }
}
