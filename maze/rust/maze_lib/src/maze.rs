use rand::rngs::ThreadRng;
use rand::Rng;
use std::fmt;

use crate::algorithms::Point;
use crate::MazeCell;
use crate::SlimWallsCell;
use crate::SlimWallsCellType;
use crate::ThickMazeCell;
use crate::ThickMazeCellType;

#[derive(Clone, Copy, Default, Debug)]
pub enum MazeType {
    #[default]
    Thick,
    Slim,
}

enum CellMarkType {
    Visited,
    Path,
    FinalPath,
    Entrance,
    Exit,
}

#[derive(Clone, Default)]
pub struct Maze {
    pub maze_type: MazeType,
    pub width: usize,
    pub height: usize,
    cells: Vec<MazeCell>,
    original_cells: Vec<MazeCell>,
}

impl Maze {
    pub fn new(
        width: usize,
        height: usize,
        maze_type: MazeType,
        default_cell_type: Option<MazeCell>,
    ) -> Self {
        let default_cell_type = default_cell_type.unwrap_or_else(|| match maze_type {
            MazeType::Thick => MazeCell::Thick(ThickMazeCell::default()),
            MazeType::Slim => MazeCell::Slim(SlimWallsCell::default()),
        });

        let cells = match maze_type {
            MazeType::Thick => vec![default_cell_type; width * height],
            MazeType::Slim => vec![default_cell_type; width * height],
        };
        let original_cells = cells.clone();

        Maze {
            maze_type,
            width,
            height,
            cells,
            original_cells,
        }
    }

    pub fn from_original(&self) -> Self {
        Maze {
            maze_type: self.maze_type,
            width: self.width,
            height: self.height,
            cells: self.original_cells.clone(),
            original_cells: self.original_cells.clone(),
        }
    }

    pub fn reset(&mut self) {
        self.cells = self.original_cells.clone();
    }

    pub fn backup(&mut self) {
        self.original_cells = self.cells.clone();
    }

    pub fn get_entrance(&self) -> Option<Point> {
        // Find and return the entrance coordinated
        self.original_cells
            .iter()
            .enumerate()
            .find_map(|(index, &cell)| {
                if cell.is_entrance() {
                    Some(Point {
                        x: index % self.width,
                        y: index / self.width,
                    })
                } else {
                    None
                }
            })
    }

    pub fn get_exit(&self) -> Option<Point> {
        // Find and return the entrance coordinated
        self.original_cells
            .iter()
            .enumerate()
            .find_map(|(index, &cell)| {
                if cell.is_exit() {
                    Some(Point {
                        x: index % self.width,
                        y: index / self.width,
                    })
                } else {
                    None
                }
            })
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get_cell(&self, point: Point) -> MazeCell {
        self.cells[self.get_index(point.x, point.y)].clone()
    }

    pub fn set_cell(&mut self, x: usize, y: usize, value: MazeCell) {
        let index = self.get_index(x, y);
        self.cells[index] = value;
    }

    fn mark_cell(&mut self, point: Point, cell_type: CellMarkType) {
        let index = self.get_index(point.x, point.y);
        if let Some(cell) = self.cells.get_mut(index) {
            match cell {
                MazeCell::Thick(ref mut thick_walls_cell) => {
                    thick_walls_cell.cell = match cell_type {
                        CellMarkType::Visited => ThickMazeCellType::Visited,
                        CellMarkType::Path => ThickMazeCellType::Path,
                        CellMarkType::FinalPath => ThickMazeCellType::FinalPath,
                        CellMarkType::Entrance => ThickMazeCellType::Entrance,
                        CellMarkType::Exit => ThickMazeCellType::Exit,
                    };
                }
                MazeCell::Slim(ref mut slim_walls_cell) => {
                    slim_walls_cell.cell = match cell_type {
                        CellMarkType::Visited => SlimWallsCellType::Visited,
                        CellMarkType::Path => SlimWallsCellType::Path,
                        CellMarkType::FinalPath => SlimWallsCellType::FinalPath,
                        CellMarkType::Entrance => SlimWallsCellType::Entrance,
                        CellMarkType::Exit => SlimWallsCellType::Exit,
                    };
                }
            }
        }
    }

    pub fn mark_cell_as_visited(&mut self, point: Point) {
        self.mark_cell(point, CellMarkType::Visited);
    }

    pub fn mark_cell_as_path(&mut self, point: Point) {
        self.mark_cell(point, CellMarkType::Path);
    }

    pub fn mark_cell_as_final_path(&mut self, point: Point) {
        self.mark_cell(point, CellMarkType::FinalPath);
    }

    pub fn mark_cell_as_entrance(&mut self, point: Point) {
        self.mark_cell(point, CellMarkType::Entrance);
    }

    pub fn mark_cell_as_exit(&mut self, point: Point) {
        self.mark_cell(point, CellMarkType::Exit);
    }

    pub fn is_valid_coord(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32
    }

    pub fn is_not_passable(&self, current: Point, next: Point) -> bool {
        match self.get_cell(next) {
            MazeCell::Thick(thick_maze_cell) => thick_maze_cell.cell == ThickMazeCellType::Wall,
            MazeCell::Slim(slim_walls_cell) => todo!(),
        }
    }

    pub fn is_passable(&self, current: Point, next: Point) -> bool {
        !self.is_not_passable(current, next)
    }

    pub fn get_random_boundary_point(&self, rng: &mut ThreadRng) -> Point {
        let side = rng.gen_range(0..4); // 0: Top, 1: Right, 2: Bottom, 3: Left

        let (x, y) = match side {
            0 => (rng.gen_range(1..self.width - 1), 0),
            1 => (self.width - 1, rng.gen_range(1..self.height - 1)),
            2 => (rng.gen_range(1..self.width - 1), self.height - 1),
            3 => (0, rng.gen_range(1..self.height - 1)),
            _ => unreachable!(),
        };

        // Ensure at least one adjacent point is ThickMazeCellType::Path
        let adjacent_points = [
            (x, y.saturating_sub(1)),
            (x.wrapping_add(1), y),
            (x, y.wrapping_add(1)),
            (x.saturating_sub(1), y),
        ];

        if adjacent_points
            .iter()
            .any(|&(cx, cy)| match self.cells.get(self.get_index(cx, cy)) {
                Some(cell) => match cell {
                    MazeCell::Thick(thick_cell) => thick_cell.cell == ThickMazeCellType::Path,
                    MazeCell::Slim(_) => false, // Assuming SlimWallsCell does not have a path
                },
                None => false,
            })
        {
            Point { x, y }
        } else {
            self.get_random_boundary_point(rng)
        }
    }
}

impl fmt::Debug for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get_cell(Point { x, y }) {
                    MazeCell::Thick(thick_cell) => match thick_cell.cell {
                        ThickMazeCellType::Wall => write!(f, "██")?,
                        ThickMazeCellType::Path => write!(f, "  ")?,
                        ThickMazeCellType::Entrance => write!(f, " >")?,
                        ThickMazeCellType::Exit => write!(f, " E")?,
                        ThickMazeCellType::Visited => write!(f, " v")?,
                        ThickMazeCellType::FinalPath => write!(f, " F")?,
                    },
                    MazeCell::Slim(slim_cell) => match slim_cell.cell {
                        SlimWallsCellType::Path => write!(f, "  ")?,
                        SlimWallsCellType::Entrance => write!(f, " >")?,
                        SlimWallsCellType::Exit => write!(f, " E")?,
                        SlimWallsCellType::Visited => write!(f, " v")?,
                        SlimWallsCellType::FinalPath => write!(f, " F")?,
                    },
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
