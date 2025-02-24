#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ThickMazeCellType {
    #[default]
    Wall,
    Path,
    Entrance,
    Exit,
    Visited,
    FinalPath,
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct ThickMazeCell {
    pub cell: ThickMazeCellType,
}
