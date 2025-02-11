from abc import ABC, abstractmethod

from maze_lib import Cell, CellType, Directions, Maze
from maze_lib.types import Point


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

    def get_neighbors(self, col: int, row: int, valid: bool = True) \
            -> list[tuple[Point, Cell]]:
        """Returns valid neighbors for the current cell."""
        neighbors = []
        for dr, dc in Directions.get_directions():
            r, c = row + dr, col + dc
            if 1 <= r < self.maze.height - 1 and 1 <= c < self.maze.width - 1:
                cell = self.maze.grid[r][c]
                if valid:
                    if cell.cell_type.walkable > 0.0:
                        neighbors.append(((r, c), self.maze.grid[r][c]))
                else:
                    neighbors.append(((r, c), self.maze.grid[r][c]))
        return neighbors

    @abstractmethod
    def find_path(self):
        pass

    @abstractmethod
    def print_step(self):
        pass
