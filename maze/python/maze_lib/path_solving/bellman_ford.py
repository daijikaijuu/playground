import sys
from maze_lib import Cell, Maze
from maze_lib.types import Point
from .path_solving import PathSolving


class BellmanFord(PathSolving):
    """Bellman-Ford path finding algorithm"""

    def __init__(self, maze: Maze, step_delay: float, debug: bool = False):
        super().__init__(maze, step_delay, debug)
        self.distances = {}
        self.previous = {}

    def _find_path_generator(self):
        # Initialize distances
        self.distances = {self.start: 0}
        self.previous = {}
        self.visited = set()
        
        # Get all valid cells as vertices
        vertices = []
        for row in range(1, self.maze.height - 1):
            for col in range(1, self.maze.width - 1):
                cell = self.maze.grid[row][col]
                if cell.cell_type.walkable > 0.0:
                    vertices.append(((row, col), cell))

        # Main Bellman-Ford algorithm
        for _ in range(len(vertices) - 1):
            updated = False
            for vertex in vertices:
                if vertex not in self.distances:
                    continue

                (row, col), _ = vertex
                neighbors = self.get_neighbors(col, row, valid=True)
                
                for neighbor in neighbors:
                    if neighbor not in self.distances:
                        self.distances[neighbor] = float('inf')
                    
                    new_distance = self.distances[vertex] + self.get_edge_weight(vertex, neighbor)
                    if new_distance < self.distances[neighbor]:
                        self.distances[neighbor] = new_distance
                        self.previous[neighbor] = vertex
                        updated = True
                        self.mark_visited(neighbor[0])
                        self.visited.add(neighbor)
                
                yield  # Pause here to show progress
            
            if not updated:  # Early termination if no updates
                break

        # Check if finish is reachable
        if self.finish in self.distances:
            self.reconstruct_path()
            yield
            return True
    
        yield
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