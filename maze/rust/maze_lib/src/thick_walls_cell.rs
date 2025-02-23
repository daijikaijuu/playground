#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ThickMazeCell {
    Wall,
    #[default]
    Path,
    Entrance,
    Exit,
    Visited,
    FinalPath,
}
