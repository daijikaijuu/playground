import time
from abc import ABC, abstractmethod

from maze_lib import Cell, Directions, Maze
from maze_lib.types import Point


class PathSolving(ABC):
    maze: Maze
    start: tuple[Point, Cell]
    finish: tuple[Point, Cell]

    step_delay: float
    debug: bool

    def __init__(self, maze: Maze, step_delay: float, debug: bool = False):
        self.maze = maze

        self.step_delay = step_delay
        self.debug = debug

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

    def step(self) -> bool:
        """Execute one step of the pathfinding algorithm"""
        if not hasattr(self, '_find_path_iter'):
            self._find_path_iter = self._find_path_generator()
        try:
            next(self._find_path_iter)
            return True
        except StopIteration as e:
            # If the generator returned a value, use it; otherwise assume False
            return e.value if e.value is not None else False

    @abstractmethod
    def _find_path_generator(self):
        """Generator version of find_path that yields after each step"""
        pass

    def find_path(self) -> bool:
        """Run the complete pathfinding algorithm"""
        generator = self._find_path_generator()
        result = False
        
        try:
            while True:
                next(generator)
                if self.debug:
                    self.print_step()
        except StopIteration as e:
            result = bool(e.value)
        
        return result

    def print_step(self):
        time.sleep(self.step_delay)

    def mark_visited(self, point: Point):
        """Mark a cell as visited in the maze"""
        self.maze.mark_visited(point)

    def mark_path(self, point: Point):
        """Mark a cell as part of the solution path"""
        self.maze.mark_path(point)
