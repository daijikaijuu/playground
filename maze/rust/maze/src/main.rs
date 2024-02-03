use maze::{Maze, MazeCell, MazeGenerator};
use raylib::prelude::*;

mod maze;

const ROWS: usize = 40;
const COLS: usize = 40;

fn main() {
    let (mut rl, thread) = raylib::init().size(800, 800).title("Maze crawler").build();

    let mut maze = Maze::new(ROWS, COLS);
    maze.generate_maze(1, 1);
    println!("{:?}", maze);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        draw_maze(&maze, &mut d)
    }
}

fn draw_maze(maze: &Maze, d: &mut RaylibDrawHandle) {
    let cell_size: i32 = 16;
    for y in 0..maze.height {
        for x in 0..maze.width {
            let color = match maze.get_cell(x, y) {
                MazeCell::Wall => Color::LIGHTGRAY,
                MazeCell::Path => Color::BLACK,
                _ => Color::BLACK,
            };
            d.draw_rectangle(
                x as i32 * cell_size,
                y as i32 * cell_size,
                cell_size,
                cell_size,
                color,
            );
        }
    }
}
