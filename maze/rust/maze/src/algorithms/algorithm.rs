#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Algorithm {
    #[default]
    Backtracking,
    AStar,
}

impl Algorithm {
    pub const ALL: [Algorithm; 2] = [Algorithm::Backtracking, Algorithm::AStar];
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Algorithm::Backtracking => "Backtracking",
                Algorithm::AStar => "AStar",
            }
        )
    }
}
