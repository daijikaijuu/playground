#!/usr/bin/env python3
import random
import time
import os


def create_board(rows: int, cols: int) -> list[list[int]]:
    return [[random.choice([0, 1]) for _ in range(cols)] for _ in range(rows)]

def print_board(board) -> None:
    for row in board:
        print(''.join(['\x1B[48;2;0;255;0m  \x1B[0m' if cell else '\x1B[48;2;255;255;255m  \x1B[0m' for cell in row]))

def count_neighbors(board: list[list[int]], x, y):
    neighbors = [
        (x-1, y-1), (x-1, y), (x-1, y+1),
        (x, y-1),               (x, y+1),
        (x+1, y-1), (x+1, y), (x+1, y+1)
    ]
    count = 0
    for nx, ny in neighbors:
        if 0 <= nx < len(board) and 0 <= ny < len(board[0]):
            count += board[nx][ny]
    return count

def update_board(board):
    new_board = [[0] * len(board[0]) for _ in range(len(board))]
    for i in range(len(board)):
        for j in range(len(board[0])):
            neighbors = count_neighbors(board, i, j)
            if board[i][j] == 1:
                new_board[i][j] = 1 if 2 <= neighbors <= 3 else 0
            else:
                new_board[i][j] = 1 if neighbors == 3 else 0
    return new_board

def clear_screen() -> None:
    os.system('clear' if os.name == 'posix' else 'cls')

def main(rows: int, cols: int, generations: int, delay: float) -> None:
    board = create_board(rows, cols)
    for _ in range(generations):
        clear_screen()
        print_board(board)
        time.sleep(delay)
        board = update_board(board)


if __name__ == '__main__':
    rows = 20
    cols = 40
    generations = 100
    delay = 0.1
    main(rows, cols, generations, delay)
