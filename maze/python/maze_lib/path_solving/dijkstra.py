import heapq
import sys

from maze_lib import Cell, Maze
from maze_lib.types import Point

from .path_solving import PathSolving


class Dijkstra(PathSolving):
    """Dijkstra path finding algorithm"""

    def __init__(self, maze: Maze, step_delay: float, debug: bool = False):
        super().__init__(maze, step_delay, debug)
        self.priority_queue = []
        self.previous = {}
        self.distances = {}

    def _find_path_generator(self):
        self.priority_queue = [(0, self.start)]
        self.visited = set()
        self.distances = {self.start: 0}
        self.previous = {}

        while self.priority_queue:
            current_distance, current_node = heapq.heappop(self.priority_queue)
            if current_node in self.visited:
                continue
                
            self.visited.add(current_node)
            self.mark_visited(current_node[0])

            if current_node == self.finish:
                self.reconstruct_path()
                return True

            (row, col), cell = current_node
            neighbors = self.get_neighbors(col, row, valid=True)
            for neighbor in neighbors:
                if neighbor in self.visited:
                    continue

                neighbor_distance = current_distance + self.get_edge_weight(current_node, neighbor)
                if neighbor not in self.distances or neighbor_distance < self.distances[neighbor]:
                    self.distances[neighbor] = neighbor_distance
                    self.previous[neighbor] = current_node
                    heapq.heappush(self.priority_queue, (neighbor_distance, neighbor))
            
            yield  # Pause here to show progress
        
        return False

    def get_edge_weight(self, current_node: tuple[Point, Cell],
                        neighbor: tuple[Point, Cell]) -> float:
        current_node_weight = current_node[1].cell_type.walkable
        return 1.0 + (1.0 - current_node_weight)

    def reconstruct_path(self):
        current = self.finish
        while current in self.previous:
            self.mark_path(current[0])  # Mark cell as part of path
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
                    print(cell.cell_type.graphic, end='')  # Print cell type
            print()
        super().print_step()
