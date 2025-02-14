import sys

from maze_lib import Cell, Maze
from maze_lib.types import Point

from .path_solving import PathSolving


class Backtracking(PathSolving):
    """Backtracking path finding algorithm"""

    def __init__(self, maze: Maze, step_delay: float, debug: bool = False):
        super().__init__(maze, step_delay, debug)
        self.stack = []
        self.previous = {}

    def _find_path_generator(self):
        self.stack = [self.start]
        self.visited = set([self.start])  # Add start to visited immediately
        self.previous = {self.start: None}

        while self.stack:
            current = self.stack[-1]  # Peek at the top of the stack
            
            if current == self.finish:
                self.reconstruct_path()
                yield
                return True

            (row, col), _ = current
            neighbors = self.get_neighbors(col, row, valid=True)
            unvisited_neighbors = [n for n in neighbors if n not in self.visited]

            if unvisited_neighbors:
                # Move to an unvisited neighbor
                next_cell = unvisited_neighbors[0]
                self.visited.add(next_cell)
                self.mark_visited(next_cell[0])
                self.previous[next_cell] = current
                self.stack.append(next_cell)
            else:
                # No unvisited neighbors - backtrack
                backtrack_point = self.stack.pop()
                if backtrack_point != self.start:  # Don't mark start as discarded
                    self.mark_discarded(backtrack_point[0])

            yield  # Pause here to show progress

        # If we get here, we've exhausted all possibilities without finding the finish
        yield
        return False  # No path found

    def reconstruct_path(self):
        """Reconstruct and mark the solution path"""
        current = self.finish
        while current in self.previous:
            self.mark_path(current[0])
            current = self.previous[current]
        self.mark_path(self.start[0])  # Mark start position

    def print_step(self):
        if sys.stdout.isatty():
            print('\033[2J\033[H')  # Clear screen
        for r, row in enumerate(self.maze.grid):
            for c, cell in enumerate(row):
                if cell.in_path:
                    print('\033[1;30;42m#\033[1;0m', end='')
                elif cell.visited:
                    print('\033[1;30;44m*\033[1;0m', end='')
                else:
                    print(cell.cell_type.graphic, end='')
            print()
        super().print_step()
