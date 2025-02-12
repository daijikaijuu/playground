import heapq
import sys

from maze_lib import Maze

from .path_solving import PathSolving


class Dijkstra(PathSolving):
    """Dijkstra path finding algorithm"""

    def __init__(self, maze: Maze, step_delay: float, debug: bool = False):
        super().__init__(maze, step_delay, debug)
        self.path = []

    def find_path(self) -> bool:
        # Priority queue to store (distance, current_node)
        priority_queue = [(0, self.start)]
        self.visited = set()
        self.distances = {self.start: 0}
        self.previous = {}

        while priority_queue:
            current_distance, current_node = heapq.heappop(priority_queue)
            if current_node in self.visited:
                continue
            self.visited.add(current_node)

            if self.debug:
                self.print_step()

            if current_node == self.finish:
                self.reconstruct_path()
                return True

            (row, col), cell = current_node
            neighbors = self.get_neighbors(col, row, valid=True)
            for neighbor in neighbors:
                if neighbor in self.visited:
                    continue

                neighbor_distance = current_distance + \
                    self.get_edge_weight(current_node, neighbor)
                if neighbor not in self.distances or neighbor_distance < self.distances[neighbor]:
                    self.distances[neighbor] = neighbor_distance
                    self.previous[neighbor] = current_node
                    heapq.heappush(
                        priority_queue, (neighbor_distance, neighbor))
        return False

    def get_edge_weight(self, current_node, neighbor):
        print(current_node)
        print(neighbor)
        return 1

    def reconstruct_path(self):
        current = self.finish
        path = []
        while current in self.previous:
            path.append(current)
            current = self.previous[current]
        path.append(self.start)
        path.reverse()
        self.path = path        # if sys.stdout.isatty():
        self.print_step()

    def print_step(self):
        if sys.stdout.isatty():
            print('\033[2J\033[H')  # Clear screen
        for r, row in enumerate(self.maze.grid):
            for c, cell in enumerate(row):
                if ((r, c), cell) in self.path:
                    print('\033[1;30;42m#\033[1;0m', end='')
                elif ((r, c), cell) in self.visited:
                    print('\033[1;30;44m*\033[1;0m', end='')
                else:
                    print(cell.cell_type.graphic, end='')  # Print cell type
            print()
        super().print_step()
