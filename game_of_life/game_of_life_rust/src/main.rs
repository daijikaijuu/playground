use crossterm::{cursor, execute, terminal};
use std::io::stdout;
use std::{thread, time};

const ROWS: usize = 20;
const COLS: usize = 20;

fn main() {
    let mut board = initialize_board();
    print_board(&board);

    loop {
        update_board(&mut board);
        print_board_inline(&board);
        thread::sleep(time::Duration::from_millis(500));
    }
}

fn initialize_board() -> [[bool; COLS]; ROWS] {
    let mut board = [[false; COLS]; ROWS];

    board[0][1] = true;
    board[1][2] = true;
    board[2][0] = true;
    board[2][1] = true;
    board[2][2] = true;

    board
}

fn print_board(board: &[[bool; COLS]; ROWS]) {
    for row in board.iter() {
        for &cell in row.iter() {
            if cell {
                print!("\x1B[48;2;0;255;0m  \x1B[0m");
            } else {
                print!("\x1B[48;2;255;255;255m  \x1B[0m");
            }
        }
        println!();
    }
}

fn print_board_inline(board: &[[bool; COLS]; ROWS]) {
    execute!(
        stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .unwrap();

    print_board(board);
}

fn count_neighbors(board: &[[bool; COLS]; ROWS], row: usize, col: usize) -> u8 {
    let mut count = 0;

    for i in (row as isize - 1)..=(row as isize + 1) {
        for j in (col as isize - 1)..=(col as isize + 1) {
            if i >= 0
                && i < ROWS as isize
                && j >= 0
                && j < COLS as isize
                && !(i == row as isize && j == col as isize)
            {
                if board[i as usize][j as usize] {
                    count += 1
                }
            }
        }
    }

    count
}

fn update_board(board: &mut [[bool; COLS]; ROWS]) {
    let mut new_board = [[false; COLS]; ROWS];

    for i in 0..ROWS {
        for j in 0..COLS {
            let neighbors = count_neighbors(board, i, j);

            new_board[i][j] = match (board[i][j], neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }

    *board = new_board;
}
