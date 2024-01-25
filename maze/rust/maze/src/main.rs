use rand::seq::SliceRandom;
// use std::{thread, time};

const ROWS: usize = 20;
const COLS: usize = 20;

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

type Maze = [[Cell; COLS]; ROWS];

fn main() {
    let mut maze = Maze::default();
    generate_maze(0, 0, &mut maze);

    print_maze(&maze);
}

fn generate_maze(row: usize, col: usize, maze: &mut Maze) {
    let mut directions = [(0, 1), (1, 0), (0, !0), (!0, 0)];
    let mut rng = rand::thread_rng();

    directions.shuffle(&mut rng);

    for &(dx, dy) in directions.iter() {
        let new_row = row.wrapping_add((dy as usize).checked_mul(2).unwrap_or(2));
        let new_col = col.wrapping_add((dx as usize).checked_mul(2).unwrap_or(2));

        if new_row < ROWS && new_col < COLS && maze[new_row][new_col] == Cell::Wall {
            // maze[row.wrapping_add(dy as usize)][col.wrapping_add(dx as usize)] = Cell::Path;
            maze[new_row][new_col] = Cell::Path;

            generate_maze(new_row, new_col, maze);
        }
    }
}

fn print_maze(maze: &Maze) {
    for row in maze.iter() {
        for &cell in row.iter() {
            match cell {
                Cell::Wall => print!("■ "),
                Cell::Path => print!("1 "),
            }
        }
        println!();
    }
}
