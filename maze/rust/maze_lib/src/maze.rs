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

//impl MazeGenerator for Maze {
//    fn generate_maze(&mut self, start_x: usize, start_y: usize) {
//        self.cells = vec![MazeCell::Wall; self.width * self.height];
//
//        let mut rng = rand::thread_rng();
//        let directions = [(0, -2), (0, 2), (-2, 0), (2, 0)];
//
//        self.set_cell(start_x, start_y, MazeCell::Path);
//        MazeGenerator::depth_first_search(
//            self,
//            start_x as i32,
//            start_y as i32,
//            &mut rng,
//            &directions,
//        );
//
//        self.set_cell(start_x, start_y, MazeCell::Entrance);
//        // Find a random point on the boundary as the exit point
//        let exit_point = self.get_random_boundary_point(&mut rng);
//        self.set_cell(exit_point.0, exit_point.1, MazeCell::Exit);
//
//        // Backup generated maze
//        self.original_cells = self.cells.clone();
//    }
//
//    fn prims_algorithm(&mut self, start_x: usize, start_y: usize, rng: &mut ThreadRng) {
//        let mut frontier = Vec::new();
//        let initial_cell = (start_x, start_y);
//        frontier.push(initial_cell);
//        self.set_cell(start_x, start_y, MazeCell::Path);
//
//        while !frontier.is_empty() {
//            let current_cell = *frontier.choose(rng).expect("Frontier is empty");
//            frontier.retain(|&cell| cell != current_cell);
//
//            let mut valid_neighbors = Vec::new();
//
//            for &(dx, dy) in &[(0, -2), (0, 2), (-2, 0), (2, 0)] {
//                let nx = current_cell.0 as i32 + dx;
//                let ny = current_cell.1 as i32 + dy;
//
//                if self.is_valid_move(nx, ny) {
//                    let neighbor_cell = (nx as usize, ny as usize);
//
//                    if !frontier.contains(&neighbor_cell)
//                        && self.get_cell(nx as usize, ny as usize) == MazeCell::Wall
//                    {
//                        valid_neighbors.push(neighbor_cell);
//                    }
//                }
//            }
//
//            if let Some(&new_cell) = valid_neighbors.choose(rng) {
//                let (cx, cy) = current_cell;
//                let (nx, ny) = new_cell;
//
//                self.set_cell(nx, ny, MazeCell::Path);
//                self.set_cell((cx + nx) / 2, (cy + ny) / 2, MazeCell::Path);
//
//                frontier.push(new_cell);
//            }
//        }
//    }
//}
//
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
