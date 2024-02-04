use maze::{Maze, MazeCell, MazeGenerator};
use raylib::prelude::*;

mod maze;

const ROWS: usize = 41;
const COLS: usize = 41;

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
    let cell_width: i32 = d.get_screen_width() / (maze.width as i32);
    let cell_height: i32 = d.get_screen_height() / maze.height as i32;
    for y in 0..maze.height {
        for x in 0..maze.width {
            let color = match maze.get_cell(x, y) {
                MazeCell::Wall => Color::LIGHTGRAY,
                MazeCell::Path => Color::BLACK,
                MazeCell::Entrance => Color::BLUE,
                MazeCell::Exit => Color::RED,
            };
            d.draw_rectangle(
                x as i32 * cell_width,
                y as i32 * cell_height,
                cell_width,
                cell_height,
                color,
            );
        }
    }
}
