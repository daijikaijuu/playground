use rand::rngs::ThreadRng;
use rand::Rng;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum MazeCell {
    Wall,
    #[default]
    Path,
    Entrance,
    Exit,
    Visited,
    FinalPath,
}

#[derive(Clone, Default)]
pub struct Maze {
    pub width: usize,
    pub height: usize,
    cells: Vec<MazeCell>,
    original_cells: Vec<MazeCell>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![MazeCell::Wall; width * height];
        let original_cells = vec![MazeCell::Wall; width * height];
        Maze {
            width,
            height,
            cells,
            original_cells,
        }
    }

    pub fn from_original(&self) -> Self {
        Maze {
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

    pub fn get_entrance(&self) -> Option<(usize, usize)> {
        // Find and return the entrance coordinated
        self.original_cells
            .iter()
            .enumerate()
            .find_map(|(index, &cell)| {
                if cell == MazeCell::Entrance {
                    Some((index % self.width, index / self.width))
                } else {
                    None
                }
            })
    }

    pub fn get_exit(&self) -> Option<(usize, usize)> {
        // Find and return the entrance coordinated
        self.original_cells
            .iter()
            .enumerate()
            .find_map(|(index, &cell)| {
                if cell == MazeCell::Exit {
                    Some((index % self.width, index / self.width))
                } else {
                    None
                }
            })
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get_cell(&self, x: usize, y: usize) -> MazeCell {
        self.cells[self.get_index(x, y)]
    }

    pub fn set_cell(&mut self, x: usize, y: usize, value: MazeCell) {
        let index = self.get_index(x, y);
        self.cells[index] = value;
    }

    pub fn is_valid_move(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32
    }

    pub fn get_random_boundary_point(&self, rng: &mut ThreadRng) -> (usize, usize) {
        let side = rng.gen_range(0..4); // 0: Top, 1: Right, 2: Bottom, 3: Left

        let (x, y) = match side {
            0 => (rng.gen_range(1..self.width - 1), 0),
            1 => (self.width - 1, rng.gen_range(1..self.height - 1)),
            2 => (rng.gen_range(1..self.width - 1), self.height - 1),
            3 => (0, rng.gen_range(1..self.height - 1)),
            _ => unreachable!(),
        };

        // Ensure at least one adjacent point is MazeCell::Path
        let adjacent_points = [
            (x, y.saturating_sub(1)),
            (x.wrapping_add(1), y),
            (x, y.wrapping_add(1)),
            (x.saturating_sub(1), y),
        ];

        if adjacent_points
            .iter()
            .any(|&(cx, cy)| match self.cells.get(self.get_index(cx, cy)) {
                Some(x) => *x == MazeCell::Path,
                None => false,
            })
        {
            (x, y)
        } else {
            self.get_random_boundary_point(rng)
        }
    }
}

impl fmt::Debug for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get_cell(x, y) {
                    MazeCell::Wall => write!(f, "██")?,
                    MazeCell::Path => write!(f, "  ")?,
                    MazeCell::Entrance => write!(f, " >")?,
                    MazeCell::Exit => write!(f, " E")?,
                    MazeCell::Visited => write!(f, " v")?,
                    MazeCell::FinalPath => write!(f, " F")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
