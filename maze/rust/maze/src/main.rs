use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};

use raylib::prelude::*;

const ROWS: usize = 40;
const COLS: usize = 40;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Wall,
    Path,
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Wall
    }
}

struct Maze {
    cells: [[Cell; COLS]; ROWS],
}

impl Maze {
    fn fill_maze_with(&mut self) {
        for i in 0..ROWS {
            for j in 0..COLS {
                if (i + j) % 2 == 0 {
                    self.cells[i][j] = Cell::Path;
                }
            }
        }
    }
}

impl Default for Maze {
    fn default() -> Self {
        Maze {
            cells: [[Cell::default(); COLS]; ROWS],
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().size(800, 800).title("Maze crawler").build();

    let mut maze = Maze::default();
    maze.fill_maze_with();
    for _ in 0..20 {
        generate_maze(&mut maze);
    }

    print_maze(&maze);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        draw_maze(&maze, &mut d)
    }
}

fn generate_maze(maze: &mut Maze) {
    let mut sets: HashMap<usize, HashSet<usize>> = HashMap::new();
    for row in 0..ROWS {
        for col in 0..COLS {
            if !sets.contains_key(&col) {
                sets.insert(col, HashSet::new());
            }
            sets.get_mut(&col).unwrap().insert(row);
            if col > 0 && should_carve(&sets, col) {
                carve_horizontaly(&mut sets, col, row, maze);
            }
            if row > 0 && should_carve(&sets, col) {
                carve_vertically(&mut sets, col, row, maze);
            }
        }
        sets.retain(|_, v| !v.is_empty());
    }
}

fn should_carve(sets: &HashMap<usize, HashSet<usize>>, col: usize) -> bool {
    if let Some(current_set) = sets.get(&col) {
        if let Some(next_set) = sets.get(&(col + 1)) {
            return !current_set.is_disjoint(next_set);
        }
    }
    false
}

fn carve_horizontaly(
    sets: &mut HashMap<usize, HashSet<usize>>,
    col: usize,
    row: usize,
    maze: &mut Maze,
) {
    let current_set = sets.get(&col).unwrap().clone();
    let next_set = sets.get(&(col + 1)).unwrap().clone();
    let mut intersection = Vec::new();
    for &element in &current_set {
        if next_set.contains(&element) {
            intersection.push(element);
        }
    }
    let mut rng = rand::thread_rng();
    let remove_wall = *intersection.choose(&mut rng).unwrap_or(&0);
    maze.cells[remove_wall][col] = Cell::Path;
    sets.get_mut(&col).unwrap().insert(row);
    sets.get_mut(&(col + 1)).unwrap().insert(row);
    sets.get_mut(&col).unwrap().extend(next_set);
    sets.remove(&(col + 1));
}

fn carve_vertically(
    sets: &mut HashMap<usize, HashSet<usize>>,
    col: usize,
    row: usize,
    maze: &mut Maze,
) {
    let current_set = sets.get(&col).unwrap().clone();
    let next_set = sets.get(&(col + 1)).unwrap().clone();
    let available_rows: Vec<_> = current_set.iter().cloned().collect();
    let mut rng = rand::thread_rng();
    let remove_wall = available_rows.choose(&mut rng).unwrap();
    maze.cells[row][*remove_wall] = Cell::Path;
    sets.get_mut(&col).unwrap().insert(row);
    sets.get_mut(&col).unwrap().extend(next_set.clone());
    sets.get_mut(&(col + 1)).unwrap().extend(next_set);
    sets.remove(&(col + 1));
}

fn print_maze(maze: &Maze) {
    for row in maze.cells.iter() {
        for &cell in row.iter() {
            match cell {
                Cell::Wall => print!("■ "),
                Cell::Path => print!("1 "),
            }
        }
        println!();
    }
}

fn draw_maze(maze: &Maze, d: &mut RaylibDrawHandle) {
    let cell_size: i32 = 16;
    for (i, row) in maze.cells.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            let color = match cell {
                Cell::Wall => Color::LIGHTGRAY,
                Cell::Path => Color::LIME,
            };
            d.draw_rectangle(
                i as i32 * cell_size,
                j as i32 * cell_size,
                cell_size,
                cell_size,
                color,
            );
        }
    }
}
