import heapq
import sys

from maze_lib import Cell, Maze
from maze_lib.types import Point

from .path_solving import PathSolving


class AStar(PathSolving):
    """A* path finding algorithm"""

    def __init__(self, maze: Maze, step_delay: float, debug: bool = False):
        super().__init__(maze, step_delay, debug)
        self.priority_queue = []
        self.previous = {}
        self.g_costs = {}
        self.f_costs = {}

    def _find_path_generator(self):
        self.priority_queue = [(0, 0, self.start)]
        self.visited = set()
        self.g_costs = {self.start: 0}
        self.f_costs = {self.start: 0}
        self.previous = {}

        while self.priority_queue:
            _, current_g_cost, current_node = heapq.heappop(
                self.priority_queue)
            if current_node in self.visited:
                continue

            self.visited.add(current_node)
            self.mark_visited(current_node[0])

            if current_node == self.finish:
                self.reconstruct_path()
                return True

            (row, col), _ = current_node
            neighbors = self.get_neighbors(col, row, valid=True)
            for neighbor in neighbors:
                if neighbor in self.visited:
                    continue

                neighbor_g_cost = current_g_cost + \
                    self.get_edge_weight(current_node, neighbor)
                neighbor_h_cost = self.heuristic(neighbor)
                neighbor_f_cost = neighbor_g_cost + neighbor_h_cost

                if neighbor not in self.g_costs or neighbor_g_cost < self.g_costs[neighbor]:
                    self.g_costs[neighbor] = neighbor_g_cost
                    self.f_costs[neighbor] = neighbor_f_cost
                    self.previous[neighbor] = current_node
                    heapq.heappush(self.priority_queue,
                                   (neighbor_f_cost, neighbor_g_cost, neighbor))

            yield  # Pause here to show progress

        return False

    def get_edge_weight(self, current_node: tuple[Point, Cell],
                        neighbor: tuple[Point, Cell]) -> float:
        current_node_weight = current_node[1].cell_type.walkable
        return 1.0 + (1.0 - current_node_weight)

    def heuristic(self, node: tuple[Point, Cell]) -> float:
        (r, c), _ = node
        (fr, fc), _ = self.finish
        return abs(r - fr) + abs(c - fc)

    def reconstruct_path(self):
        current = self.finish
        while current in self.previous:
            self.mark_path(current[0])  # Mark cell as part of path
            current = self.previous[current]
        self.mark_path(self.start[0])  # Mark start position
        if self.debug:
            self.print_step()

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
