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

            allowed_types = set()
            direction = Directions.calculate_direction((row, col), (nr, nc))
            allowed_types = self.get_allowed_types(
                row, col, current_type, direction)
            print((row, col), current_type, direction, (nr, nc), allowed_types)
            neighbor_cell.possible_types &= allowed_types
            print(neighbor_cell.possible_types)

            if not neighbor_cell.possible_types:
                raise ValueError("No possible types for neighbor cell")

    def get_allowed_types(self, col: int, row: int, current_type: CellType, direction: Directions) -> set[CellType]:
        """Determine which types are allowed for the neighbor based on the
           current cell.
        """
        match (current_type, direction):
            case (CellType.WALL_HORIZONTAL, Directions.UP | Directions.DOWN):
                return {CellType.FLOOR}
            case (CellType.WALL_HORIZONTAL, Directions.LEFT):
                return {CellType.WALL_HORIZONTAL,
                        CellType.WALL_CORNER_TL,
                        CellType.WALL_CORNER_BL}
            case (CellType.WALL_HORIZONTAL, Directions.RIGHT):
                return {CellType.WALL_HORIZONTAL,
                        CellType.WALL_CORNER_TR,
                        CellType.WALL_CORNER_BR}

            case (CellType.WALL_VERTICAL, Directions.LEFT | Directions.RIGHT):
                return {CellType.FLOOR}
            case (CellType.WALL_VERTICAL, Directions.UP):
                return {CellType.WALL_VERTICAL,
                        CellType.WALL_CORNER_TL,
                        CellType.WALL_CORNER_TR}
            case (CellType.WALL_VERTICAL, Directions.DOWN):
                return {CellType.WALL_VERTICAL,
                        CellType.WALL_CORNER_BL,
                        CellType.WALL_CORNER_BR}

            case (CellType.WALL_CORNER_TL, Directions.UP | Directions.LEFT):
                return {CellType.FLOOR}
            case (CellType.WALL_CORNER_TL, Directions.RIGHT):
                return {CellType.WALL_HORIZONTAL,
                        CellType.WALL_CORNER_BR}
            case (CellType.WALL_CORNER_TL, Directions.DOWN):
                return {CellType.WALL_VERTICAL,
                        CellType.WALL_CORNER_BL,
                        CellType.WALL_CORNER_BR}

            case (CellType.WALL_CORNER_TR, Directions.UP | Directions.RIGHT):
                return {CellType.FLOOR}
            case (CellType.WALL_CORNER_TR, Directions.LEFT):
                return {CellType.WALL_HORIZONTAL,
                        CellType.WALL_CORNER_BL}
            case (CellType.WALL_CORNER_TR, Directions.DOWN):
                return {CellType.WALL_VERTICAL,
                        CellType.WALL_CORNER_BL}

            case (CellType.WALL_CORNER_BL, Directions.LEFT | Directions.DOWN):
                return {CellType.FLOOR}
            case (CellType.WALL_CORNER_BL, Directions.RIGHT):
                return {CellType.WALL_HORIZONTAL,
                        CellType.WALL_CORNER_TR}
            case (CellType.WALL_CORNER_BL, Directions.UP):
                return {CellType.WALL_VERTICAL,
                        CellType.WALL_CORNER_TL,
                        CellType.WALL_CORNER_TR}

            case (CellType.WALL_CORNER_BR, Directions.RIGHT | Directions.DOWN):
                return {CellType.FLOOR}
            case (CellType.WALL_CORNER_BR, Directions.LEFT):
                return {CellType.WALL_HORIZONTAL,
                        CellType.WALL_CORNER_TL}
            case (CellType.WALL_CORNER_BR, Directions.UP):
                return {CellType.WALL_VERTICAL,
                        CellType.WALL_CORNER_TL}

            case (CellType.WALL_T_CROSS, Directions.UP):
                return {CellType.FLOOR}
            case (CellType.WALL_T_CROSS, Directions.DOWN):
                return {CellType.WALL_VERTICAL}

            case _:
                # Default: Allow all types if no specific rule applies
                return set(CellType)

    def find_min_entropy_cell(self) -> tuple[int, int] | None:
        """Find the cell with the minimum entropy (least number of possible types)"""
        min_entropy = float('inf')
        min_cell = None
        for r in range(self.height):
            for c in range(self.width):
                cell = self.grid[r][c]
                if not cell.is_collapsed():
                    entropy = len(cell.possible_types)
                    if 1 <= entropy < min_entropy:
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
        while not self.is_fully_collapsed():
            min_cell = self.find_min_entropy_cell()
            if min_cell:
                r, c = min_cell
                self.print_maze(r, c)
                cell = self.grid[r][c]
                print(f'min entropy cell: {min_cell}', cell.possible_types)
                cell.collapse(random.choice(list(cell.possible_types)))
                self.propagate_constraints(r, c)
            else:
                print('random cell')
                self.collapse_random_cell()
            print('------')

    def print_maze(self, hr: int = -1, hc: int = -1):
        """Print the generated maze with a gradient based on entropy."""
        # Clear the screen and move the cursor to the top-left corner
        # Uncomment the following line if you want to clear the screen
        # print("\033[2J\033[H", end='')

        max_entropy = len(CellType)

        for r, row in enumerate(self.grid):
            for c, cell in enumerate(row):
                entropy = len(cell.possible_types)
                i = cell.cell_type.graphic if cell.cell_type else '?'

                # Determine the color based on entropy
                if r == hr and c == hc:
                    # Highlight the current cell in red
                    print(f"\033[5m{i}\033[0m", end='')  # Red color
                else:
                    # Map entropy to a grayscale gradient (30-37 are ANSI grayscale codes)
                    if entropy > 0:
                        # Scale entropy to 30-37
                        gray_level = int(30 + (entropy / max_entropy) * 7)
                        # Clamp to valid range
                        gray_level = min(max(gray_level, 30), 37)
                        # print(gray_level)
                        print(f"\033[{gray_level}m{i}\033[0m",
                              end='')  # Apply grayscale
                    else:
                        # Default color if max_entropy is 0
                        print(f'\033[47m{i}\033[0m', end='')
            print('')  # Newline after each row
