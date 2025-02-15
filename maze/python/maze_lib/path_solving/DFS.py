import sys

from maze_lib import Maze

from .path_solving import PathSolving


class DFS(PathSolving):
    def __init__(self, maze: Maze, step_delay: float, debug: bool = False):
        super().__init__(maze, step_delay, debug)
        self.stack = []
        self.previous = {}

    def _find_path_generator(self):
        self.stack = [self.start]
        self.visited = set()
        self.previous = {self.start: None}

        while self.stack:
            current = self.stack.pop()
            if current in self.visited:
                continue

            self.visited.add(current)
            self.mark_visited(current[0])

            if current == self.finish:
                self.reconstruct_path()
                return True

            (row, col), _ = current
            neighbors = self.get_neighbors(col, row, valid=True)
            for neighbor in neighbors:
                if neighbor not in self.visited:
                    self.previous[neighbor] = current
                    self.stack.append(neighbor)

            yield  # Pause here to show progress

        return False

    def reconstruct_path(self):
        """Reconstruct and mark the solution path"""
        current = self.finish
        while current in self.previous:
            self.mark_path(current[0])
            current = self.previous[current]
        self.mark_path(self.start[0])  # Mark start position

    def print_step(self):
        if sys.stdout.isatty():
            print('\033[2J\033[H')
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
