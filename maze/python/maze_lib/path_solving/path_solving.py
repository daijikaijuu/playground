from abc import ABC, abstractmethod

from maze_lib import Cell, Directions, Maze, Point


class PathSolving(ABC):
    maze: Maze
    start: tuple[Point, Cell]
    finish: tuple[Point, Cell]

    def __init__(self, maze: Maze):
        self.maze = maze

        start, finish = self.maze.find_start_and_finish()
        self.start = start, self.maze.grid[start[0]][start[1]]
        self.finish = finish, self.maze.grid[finish[0]][finish[1]]

    def get_cell(self, col: int, row: int) -> tuple[Point, Cell]:
        return (col, row), self.maze.grid[row][col]

    def get_neighbors(self, col: int, row: int) -> list[tuple[Point, Cell]]:
        """Returns valid neighbors for the current cell."""
        neighbors = []
        for dr, dc in Directions.get_directions():
            r, c = row + dr, col + dc
            if 0 <= r < self.maze.height and 0 <= c < self.maze.width:
                neighbors.append(((r, c), self.maze.grid[r][c]))
        return neighbors

    def is_valid_path(self, col: int, row: int) -> bool:
        cell = self.maze.grid[row][col]
        pass

    @abstractmethod
    def find_path(self):
        pass

    @abstractmethod
    def print_step(self):
        pass
