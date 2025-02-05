use std::collections::{HashMap, VecDeque};

use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    name: String,
    connects: HashMap<Direction, Vec<usize>>,
}

#[derive(Debug, Clone)]
struct Cell {
    possible_tiles: Vec<usize>,
    collapsed: bool,
}

struct WfcGrid {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
    tiles: Vec<Tile>,
}

impl WfcGrid {
    fn new(width: usize, height: usize, tiles: Vec<Tile>) -> Self {
        let mut cells = Vec::with_capacity(height);
        for _ in 0..height {
            let row = (0..width)
                .map(|_| Cell {
                    possible_tiles: tiles.iter().map(|t| t.id).collect(),
                    collapsed: false,
                })
                .collect();
            cells.push(row);
        }

        WfcGrid {
            width,
            height,
            cells,
            tiles,
        }
    }

    fn find_lowest_entropy(&self) -> Option<(usize, usize)> {
        let mut min_entropy = usize::MAX;
        let mut target = None;
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if !cell.collapsed {
                    let entropy = cell.possible_tiles.len();
                    if entropy < min_entropy && entropy > 0 {
                        min_entropy = entropy;
                        target = Some((x, y));
                    }
                }
            }
        }

        target
    }

    fn collapse_cell(&mut self, x: usize, y: usize) {
        let mut rng = thread_rng();
        let cell = &mut self.cells[y][x];
        if let Some(time_id) = cell.possible_tiles.choose(&mut rng) {
            cell.possible_tiles = vec![*time_id];
            cell.collapsed = true;
        }
    }

    fn propagate(&mut self, start_x: usize, start_y: usize) {
        let mut queue = VecDeque::new();
        queue.push_back((start_x, start_y));

        while let Some((x, y)) = queue.pop_front() {
            let current_id = self.cells[y][x].possible_tiles[0];
            let current_tile = self.tiles.iter().find(|t| t.id == current_id).unwrap();

            for dir in &[
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                let (nx, ny) = match dir {
                    Direction::Up => (x as isize, y as isize - 1),
                    Direction::Down => (x as isize, y as isize + 1),
                    Direction::Left => (x as isize - 1, y as isize),
                    Direction::Right => (x as isize + 1, y as isize),
                };

                if nx < 0 || ny < 0 || nx >= self.width as isize || ny >= self.height as isize {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;
                let neighbor = &mut self.cells[ny][nx];

                if neighbor.collapsed {
                    continue;
                }

                let opposite_dir = match dir {
                    Direction::Up => Direction::Down,
                    Direction::Down => Direction::Up,
                    Direction::Left => Direction::Right,
                    Direction::Right => Direction::Left,
                };

                let allowed = current_tile.connects.get(dir).unwrap();
                let mut new_possible = Vec::new();

                for tile_id in &neighbor.possible_tiles {
                    let tile = self.tiles.iter().find(|t| t.id == *tile_id).unwrap();
                    if tile
                        .connects
                        .get(&opposite_dir)
                        .unwrap()
                        .iter()
                        .any(|&id| allowed.contains(&id))
                    {
                        new_possible.push(*tile_id);
                    }
                }

                if new_possible.is_empty() {
                    panic!("Contradiction at ({}, {})", nx, ny);
                }

                if new_possible.len() != neighbor.possible_tiles.len() {
                    neighbor.possible_tiles = new_possible;
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    fn generate(&mut self) {
        while let Some((x, y)) = self.find_lowest_entropy() {
            self.collapse_cell(x, y);
            self.propagate(x, y);
        }
    }

    fn print_grid(&self) {
        for row in &self.cells {
            for cell in row {
                let symbol = match self.tiles[cell.possible_tiles[0]].name.as_str() {
                    "Wall" => '#',
                    "Path" => '.',
                    "Corner" => '+',
                    "Cross" => '╬',
                    _ => '?',
                };
                print!("{} ", symbol);
            }
            println!();
        }
    }
}
