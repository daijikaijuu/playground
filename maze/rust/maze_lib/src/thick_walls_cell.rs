#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ThickMazeCellType {
    Wall,
    #[default]
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
