use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MazeCell {
    Wall,
    Path,
    Entrance,
    Exit,
    Visited,
    FinalPath,
}

pub trait MazeGenerator {
    fn generate_maze(&mut self, start_x: usize, start_y: usize);
    fn depth_first_search(
        &mut self,
        x: i32,
        y: i32,
        rng: &mut ThreadRng,
        directions: &[(i32, i32)],
    );
}

#[derive(Clone)]
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

    fn get_random_boundary_point(&self, rng: &mut ThreadRng) -> (usize, usize) {
        let side = rng.gen_range(0..4); // 0: Top, 1: Right, 2: Bottom, 3: Left

        let (x, y) = match side {
            0 => (rng.gen_range(1..self.width - 1), 0),
            1 => (self.width - 1, rng.gen_range(1..self.height - 1)),
            2 => (rng.gen_range(1..self.width - 1), self.height - 1),
            3 => (0, rng.gen_range(1..self.height - 1)),
            _ => unreachable!(),
        };

        (x, y)
    }
}

impl MazeGenerator for Maze {
    fn generate_maze(&mut self, start_x: usize, start_y: usize) {
        let mut rng = rand::thread_rng();
        let directions = [(0, -2), (0, 2), (-2, 0), (2, 0)];

        self.set_cell(start_x, start_y, MazeCell::Path);
        MazeGenerator::depth_first_search(
            self,
            start_x as i32,
            start_y as i32,
            &mut rng,
            &directions,
        );

        self.set_cell(start_x, start_y, MazeCell::Entrance);
        // Find a random point on the boundary as the exit point
        let exit_point = self.get_random_boundary_point(&mut rng);
        self.set_cell(exit_point.0, exit_point.1, MazeCell::Exit);

        // Backup generated maze
        self.original_cells = self.cells.clone();
    }

    fn depth_first_search(
        &mut self,
        x: i32,
        y: i32,
        rng: &mut ThreadRng,
        directions: &[(i32, i32)],
    ) {
        let mut shuffled_directions = directions.to_vec();
        shuffled_directions.shuffle(rng);
        for &(dx, dy) in &shuffled_directions {
            let new_x = x + dx;
            let new_y = y + dy;

            if self.is_valid_move(new_x, new_y) {
                let nx = new_x as usize;
                let ny = new_y as usize;

                if self.get_cell(nx, ny) == MazeCell::Wall {
                    self.set_cell(nx, ny, MazeCell::Path);
                    self.set_cell(
                        (x + new_x) as usize / 2,
                        (y + new_y) as usize / 2,
                        MazeCell::Path,
                    );

                    self.depth_first_search(new_x, new_y, rng, directions);
                }
            }
        }
    }
}

impl fmt::Debug for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get_cell(x, y) {
                    MazeCell::Wall => write!(f, "#")?,
                    MazeCell::Path => write!(f, " ")?,
                    MazeCell::Entrance => write!(f, ">")?,
                    MazeCell::Exit => write!(f, "E")?,
                    MazeCell::Visited => write!(f, "v")?,
                    MazeCell::FinalPath => write!(f, "F")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
