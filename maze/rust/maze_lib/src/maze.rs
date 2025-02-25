use rand::rngs::ThreadRng;
use rand::Rng;
use std::fmt;

use crate::algorithms::Movements;
use crate::algorithms::Point;
use crate::CellType;
use crate::MazeCell;
use crate::SlimWallsCellType;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum MazeType {
    #[default]
    Thick,
    Slim,
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
        default_cell_type: Option<CellType>,
    ) -> Self {
        let default_cell_type = default_cell_type.unwrap_or(CellType::Path);

        let cells = vec![MazeCell::new(default_cell_type); width * height];
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
        self.cells[self.get_index(point.x, point.y)]
    }

    pub fn set_cell(&mut self, x: usize, y: usize, value: MazeCell) {
        let index = self.get_index(x, y);
        self.cells[index] = value;
    }

    fn mark_cell(&mut self, point: Point, cell_type: CellType) {
        let index = self.get_index(point.x, point.y);
        if let Some(cell) = self.cells.get_mut(index) {
            cell.mark_cell_as(cell_type);
        }
    }

    pub fn mark_cell_as_visited(&mut self, point: Point) {
        self.mark_cell(point, CellType::Visited);
    }

    pub fn mark_cell_as_path(&mut self, point: Point) {
        self.mark_cell(point, CellType::Path);
    }

    pub fn mark_cell_as_final_path(&mut self, point: Point) {
        self.mark_cell(point, CellType::FinalPath);
    }

    pub fn mark_cell_as_entrance(&mut self, point: Point) {
        self.mark_cell(point, CellType::Entrance);
    }

    pub fn mark_cell_as_exit(&mut self, point: Point) {
        self.mark_cell(point, CellType::Exit);
    }

    pub fn is_valid_coord(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < (self.width - 1) as i32 && y < (self.height - 1) as i32
    }

    pub fn is_not_passable(&self, current: Point, next: Point) -> bool {
        match self.maze_type {
            MazeType::Thick => self.get_cell(next).get_type() == CellType::Wall,
            MazeType::Slim => {
                let direction = Movements::calculate_direction(current, next);
                let opposite_direction =
                    Movements::get_opposite_direction(direction.0, direction.1);

                self.get_cell(current).has_wall_in_direction(direction)
                    && self
                        .get_cell(next)
                        .has_wall_in_direction(opposite_direction)
            }
        }
    }

    pub fn is_passable(&self, current: Point, next: Point) -> bool {
        !self.is_not_passable(current, next)
    }

    pub fn remove_walls_between_cells(&mut self, current: Point, neighbor: Point) {
        assert_ne!(
            self.maze_type,
            MazeType::Thick,
            "This functions should never be called in thick walls maze!"
        );

        let direction = Movements::calculate_direction(current, neighbor);
        let opposite_direction = Movements::get_opposite_direction(direction.0, direction.1);
        let current_idx = self.get_index(current.x, current.y);
        let neighbor_idx = self.get_index(neighbor.x, neighbor.y);

        if let Some(cell) = self.cells.get_mut(current_idx) {
            cell.set_wall_by_direction(direction, false);
        }
        if let Some(cell) = self.cells.get_mut(neighbor_idx) {
            cell.set_wall_by_direction(opposite_direction, false);
        }
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

        if adjacent_points.iter().any(|&(cx, cy)| {
            self.is_valid_coord(cx as i32, cy as i32)
                && self.get_cell(Point { x: cx, y: cy }).get_type() == CellType::Path
        }) {
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
                match self.get_cell(Point { x, y }).get_type() {
                    CellType::Wall => write!(f, "██")?,
                    CellType::Path => write!(f, "  ")?,
                    CellType::Entrance => write!(f, " >")?,
                    CellType::Exit => write!(f, " E")?,
                    CellType::Visited => write!(f, " v")?,
                    CellType::FinalPath => write!(f, " F")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
