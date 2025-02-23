use crate::SlimWallsCell;
use crate::SlimWallsCellType;
use crate::ThickMazeCell;
use crate::ThickMazeCellType;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MazeCell {
    Thick(ThickMazeCell),
    Slim(SlimWallsCell),
}

impl Default for MazeCell {
    fn default() -> Self {
        MazeCell::Thick(ThickMazeCell {
            cell: ThickMazeCellType::Wall,
        })
    }
}

impl MazeCell {
    pub fn is_entrance(&self) -> bool {
        match self {
            MazeCell::Thick(thick_cell) => thick_cell.cell == ThickMazeCellType::Entrance,
            MazeCell::Slim(slim_cell) => slim_cell.cell == SlimWallsCellType::Entrance,
        }
    }

    pub fn is_exit(&self) -> bool {
        match self {
            MazeCell::Thick(thick_cell) => thick_cell.cell == ThickMazeCellType::Exit,
            MazeCell::Slim(slim_cell) => slim_cell.cell == SlimWallsCellType::Exit,
        }
    }

    pub fn cell_types() -> Vec<MazeCell> {
        vec![
            MazeCell::Thick(ThickMazeCell::default()),
            MazeCell::Slim(SlimWallsCell::default()),
        ]
    }
}
