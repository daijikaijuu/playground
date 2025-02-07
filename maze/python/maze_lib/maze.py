import random

from .cell import Cell, CellType
from .directions import Directions


class Maze:
    width: int
    height: int
    grid: list[list[Cell]]

    def __init__(self, width: int, height: int):
        self.width = width
        self.height = height
        self.grid = [[Cell() for _ in range(width)] for _ in range(height)]

    def get_neighbors(self, row: int, col: int) -> list[tuple[int, int]]:
        """Get valid neighboring cells."""
        neighbors = []
        for dr, dc in Directions.get_directions():
            r, c = row + dr, col + dc
            if 0 <= r < self.height and 0 <= c < self.width:
                neighbors.append((r, c))
        return neighbors

    def propagate_constraints(self, row: int, col: int):
        """Propagate constraints to neighboring cells collapses."""
        current_cell = self.grid[row][col]
        if not current_cell.is_collapsed():
            return

        current_type = current_cell.cell_type
        neighbors = self.get_neighbors(row, col)
        for nr, nc in neighbors:
            neighbor_cell = self.grid[nr][nc]
            if neighbor_cell.is_collapsed():
                continue  # Skip already collapsed cells

            allowed_types = self.get_allowed_types(
                current_type,
                Directions.calculate_direction((row, col), (nr, nc)))
            neighbor_cell.possible_types &= allowed_types

            if not neighbor_cell.possible_types:
                raise ValueError("No possible types for neighbor cell")

    def get_allowed_types(self, current_type: CellType, direction: Directions) -> set[CellType]:
        """Determine which types are allowed for the neighbor based on the
           current cell.
        """
        match (current_type, direction):
            case (CellType.FLOOR, _):
                return {CellType.FLOOR}
            case (CellType.WALL_VERTICAL, Directions.UP | Directions.DOWN):
                return {CellType.WALL_VERTICAL}
            case (CellType.WALL_HORIZONTAL, Directions.LEFT | Directions.RIGHT):
                return {CellType.WALL_HORIZONTAL, CellType.WALL_HORIZONTAL}
            case (CellType.WALL_CORNER_TL, Directions.RIGHT):
                return {CellType.WALL_HORIZONTAL, CellType.WALL_CORNER_TR}
            case (CellType.WALL_CORNER_TR, Directions.LEFT):
                return {CellType.WALL_HORIZONTAL, CellType.WALL_CORNER_TL}
            case (CellType.WALL_CORNER_BL, Directions.RIGHT):
                return {CellType.WALL_HORIZONTAL, CellType.WALL_CORNER_BR}
            case (CellType.WALL_CORNER_BR, Directions.LEFT):
                return {CellType.WALL_HORIZONTAL, CellType.WALL_CORNER_BL}
            case (CellType.WALL_VERTICAL, Directions.UP | Directions.DOWN):
                return {CellType.WALL_VERTICAL}
            case (CellType.WALL_HORIZONTAL, Directions.LEFT | Directions.RIGHT):
                return {CellType.WALL_HORIZONTAL}
            case _:
                # Default: Allow all types if no specific rule applies
                return set(CellType)

    def find_min_entropy_cell(self) -> tuple[int, int]:
        """Find the cell with the minimum entropy (least number of possible types)"""
        min_entropy = float('inf')
        min_cell = None
        for r in range(self.height):
            for c in range(self.height):
                cell = self.grid[r][c]
                if not cell.is_collapsed():
                    entropy = len(cell.possible_types)
                    if 1 < entropy < min_entropy:
                        min_entropy = entropy
                        min_cell = (r, c)
        return min_cell

    def collapse_random_cell(self):
        """Randomly collapse a cell if no minimum entropy cell is found"""
        r = random.randint(0, self.height - 1)
        c = random.randint(0, self.width - 1)
        cell = self.grid[r][c]
        if not cell.is_collapsed():
            cell.collapse(random.choice(list(cell.possible_types)))

    def is_fully_collapsed(self) -> bool:
        """Check if all cells have been collapsed"""
        return all(cell.is_collapsed() for row in self.grid for cell in row)

    def generate_maze(self):
        """Generate the maze using Wave Function Collapse algorithm"""
        self.collapse_random_cell()

        while not self.is_fully_collapsed():
            min_cell = self.find_min_entropy_cell()
            if min_cell:
                r, c = min_cell
                cell = self.grid[r][c]
                cell.collapse(random.choice(list(cell.possible_types)))
                self.propagate_constraints(r, c)
            else:
                self.collapse_random_cell()

    def print_maze(self):
        """Print the generated maze."""
        for row in self.grid:
            print(''.join(str(cell.cell_type.graphic)
                  if cell.cell_type else '?' for cell in row))
