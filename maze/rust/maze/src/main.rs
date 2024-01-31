use rand::seq::SliceRandom;
use raylib::prelude::*;
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
    let (mut rl, thread) = raylib::init().size(800, 800).title("Maze crawler").build();

    let mut maze = Maze::default();
    generate_maze(0, 0, &mut maze);

    print_maze(&maze);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        draw_maze(&maze, &mut d)
    }
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

fn draw_maze(maze: &Maze, d: &mut RaylibDrawHandle) {
    let cell_size: i32 = 16;
    for (i, row) in maze.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            let color = match cell {
                Cell::Wall => Color::LIGHTGRAY,
                Cell::Path => Color::BLACK,
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
