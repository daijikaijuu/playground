import random

from colorama import Fore, Style

from .cell import Cell, CellType
from .directions import Directions
from .types import Point


class Maze:
    width: int
    height: int
    grid: list[list[Cell]]

    def __init__(self, width: int, height: int, debug: bool = False):
        self.width = width
        self.height = height
        self.debug = debug

    def get_neighbors(self, row: int, col: int) -> list[Point]:
        """Get valid neighboring cells."""
        neighbors = []
        for dr, dc in Directions.get_directions(diagonal=True):
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
            allowed_types = self.get_allowed_types(current_type, direction)
            # print((row, col), current_type, direction, (nr, nc), allowed_types)
            neighbor_cell.possible_types &= allowed_types
            # print("=>", neighbor_cell.possible_types)

            if not neighbor_cell.possible_types:
                raise ValueError("No possible types for neighbor cell")

    def get_allowed_types(self, current_type: CellType, direction: Directions) -> set[CellType]:
        """Determine which types are allowed for the neighbor based on the
           current cell.
        """
        match (current_type, direction):
            case (CellType.START | CellType.FINISH, Directions.UP | Directions.DOWN | Directions.LEFT | Directions.RIGHT):
                return {CellType.FLOOR}

            case (CellType.SWAMP_HEAVY, Directions.UP | Directions.DOWN | Directions.LEFT | Directions.RIGHT):
                return {CellType.SWAMP_LITE, CellType.SWAMP_MEDIUM}

            case (CellType.WALL_HORIZONTAL, Directions.UP | Directions.DOWN):
                return {CellType.FLOOR,
                        CellType.SWAMP_LITE,
                        CellType.SWAMP_MEDIUM,
                        CellType.SWAMP_HEAVY}
            case (CellType.WALL_HORIZONTAL, Directions.LEFT):
                return {CellType.WALL_HORIZONTAL,
                        CellType.WALL_CORNER_TL,
                        CellType.WALL_CORNER_BL,
                        CellType.WALL_T_CROSS,
                        CellType.WALL_B_CROSS,
                        CellType.WALL_L_CROSS,
                        CellType.WALL_CROSS,
                        CellType.FLOOR}
            case (CellType.WALL_HORIZONTAL, Directions.RIGHT):
                return {CellType.WALL_HORIZONTAL,
                        CellType.WALL_CORNER_TR,
                        CellType.WALL_CORNER_BR,
                        CellType.WALL_T_CROSS,
                        CellType.WALL_R_CROSS,
                        CellType.WALL_B_CROSS,
                        CellType.WALL_CROSS,
                        CellType.FLOOR}

            case (CellType.WALL_VERTICAL, Directions.LEFT | Directions.RIGHT):
                return {CellType.FLOOR,
                        CellType.SWAMP_LITE,
                        CellType.SWAMP_MEDIUM,
                        CellType.SWAMP_HEAVY}
            case (CellType.WALL_VERTICAL, Directions.UP):
                return {CellType.WALL_VERTICAL,
                        CellType.WALL_CORNER_TL,
                        CellType.WALL_CORNER_TR,
                        CellType.WALL_CROSS,
                        CellType.WALL_L_CROSS,
                        CellType.WALL_R_CROSS,
                        CellType.WALL_T_CROSS}
            case (CellType.WALL_VERTICAL, Directions.DOWN):
                return {CellType.WALL_VERTICAL,
                        CellType.WALL_CORNER_BL,
                        CellType.WALL_CORNER_BR,
                        CellType.WALL_CROSS,
                        CellType.WALL_L_CROSS,
                        CellType.WALL_R_CROSS,
                        CellType.WALL_B_CROSS}
            case (CellType.WALL_VERTICAL, Directions.TOP_LEFT | Directions.TOP_RIGHT | Directions.BOTTOM_LEFT | Directions.BOTTOM_RIGHT):
                return {CellType.FLOOR,
                        CellType.WALL_HORIZONTAL}

            case (CellType.WALL_CORNER_TL, Directions.UP | Directions.LEFT):
                return {CellType.FLOOR,
                        CellType.SWAMP_LITE,
                        CellType.SWAMP_MEDIUM,
                        CellType.SWAMP_HEAVY}
            case (CellType.WALL_CORNER_TL, Directions.RIGHT):
                return {CellType.WALL_HORIZONTAL,
                        CellType.WALL_CORNER_BR}
            case (CellType.WALL_CORNER_TL, Directions.DOWN):
                return {CellType.WALL_VERTICAL,
                        CellType.WALL_CORNER_BL,
                        CellType.WALL_CORNER_BR}
            case (CellType.WALL_CORNER_TL,
                  Directions.TOP_LEFT |
                  Directions.TOP_RIGHT |
                  Directions.BOTTOM_LEFT |
                  Directions.BOTTOM_RIGHT):
                return {CellType.FLOOR}

            case (CellType.WALL_CORNER_TR, Directions.UP | Directions.RIGHT):
                return {CellType.FLOOR,
                        CellType.SWAMP_LITE,
                        CellType.SWAMP_MEDIUM,
                        CellType.SWAMP_HEAVY}
            case (CellType.WALL_CORNER_TR, Directions.LEFT):
                return {CellType.WALL_HORIZONTAL,
                        CellType.WALL_CORNER_BL}
            case (CellType.WALL_CORNER_TR, Directions.DOWN):
                return {CellType.WALL_VERTICAL,
                        CellType.WALL_CORNER_BL}

            case (CellType.WALL_CORNER_BL, Directions.LEFT | Directions.DOWN):
                return {CellType.FLOOR,
                        CellType.SWAMP_LITE,
                        CellType.SWAMP_MEDIUM,
                        CellType.SWAMP_HEAVY}
            case (CellType.WALL_CORNER_BL, Directions.RIGHT):
                return {CellType.WALL_HORIZONTAL,
                        CellType.WALL_CORNER_TR}
            case (CellType.WALL_CORNER_BL, Directions.UP):
                return {CellType.WALL_VERTICAL,
                        CellType.WALL_CORNER_TL,
                        CellType.WALL_CORNER_TR}

            case (CellType.WALL_CORNER_BR, Directions.RIGHT | Directions.DOWN):
                return {CellType.FLOOR,
                        CellType.SWAMP_LITE,
                        CellType.SWAMP_MEDIUM,
                        CellType.SWAMP_HEAVY}
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
            case (CellType.WALL_T_CROSS, Directions.LEFT | Directions.RIGHT):
                return {CellType.WALL_HORIZONTAL}

            case (CellType.WALL_B_CROSS, Directions.DOWN):
                return {CellType.FLOOR}
            case (CellType.WALL_B_CROSS, Directions.UP):
                return {CellType.WALL_VERTICAL}
            case (CellType.WALL_B_CROSS, Directions.LEFT | Directions.RIGHT):
                return {CellType.WALL_HORIZONTAL}

            case (CellType.WALL_L_CROSS, Directions.LEFT):
                return {CellType.FLOOR}
            case (CellType.WALL_L_CROSS, Directions.RIGHT):
                return {CellType.WALL_HORIZONTAL}
            case (CellType.WALL_L_CROSS, Directions.UP | Directions.DOWN):
                return {CellType.WALL_VERTICAL}

            case (CellType.WALL_R_CROSS, Directions.RIGHT):
                return {CellType.FLOOR}
            case (CellType.WALL_R_CROSS, Directions.LEFT):
                return {CellType.WALL_HORIZONTAL}
            case (CellType.WALL_R_CROSS, Directions.UP | Directions.DOWN):
                return {CellType.WALL_VERTICAL}

            case (CellType.WALL_CROSS, Directions.UP | Directions.DOWN):
                return {CellType.WALL_VERTICAL}
            case (CellType.WALL_CROSS, Directions.LEFT | Directions.RIGHT):
                return {CellType.WALL_HORIZONTAL}
            case (CellType.WALL_CROSS, Directions.TOP_LEFT | Directions.TOP_RIGHT | Directions.BOTTOM_LEFT | Directions.BOTTOM_RIGHT):
                return {CellType.FLOOR}

            case _:
                # Default: Allow all types if no specific rule applies
                return set(CellType)

    def find_min_entropy_cell(self) -> Point | None:
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
            if r == 0 or r == self.height - 1 or c == 0 or c == self.width - 1:
                self.propagate_border(r, c)
            # cell.collapse(random.choice(list(cell.possible_types)))
            cell.collapse_by_frequency()

    def is_fully_collapsed(self) -> bool:
        """Check if all cells have been collapsed"""
        return all(cell.is_collapsed() for row in self.grid for cell in row)

    def propagate_border(self, row: int, col: int):
        """Collapse border cell to a specific type"""
        cell = self.grid[row][col]
        possible_types = cell.possible_types
        # print(possible_types)
        if row == 0 and col == 0:
            possible_types = {CellType.WALL_CORNER_TL}
        elif row == 0 and col == self.width - 1:
            possible_types = {CellType.WALL_CORNER_TR}
        elif row == self.height - 1 and col == 0:
            possible_types = {CellType.WALL_CORNER_BL}
        elif row == self.height - 1 and col == self.width - 1:
            possible_types = {CellType.WALL_CORNER_BR}
        elif row == 0:
            possible_types = {CellType.WALL_HORIZONTAL, CellType.WALL_T_CROSS}
        elif row == self.height - 1:
            possible_types = {CellType.WALL_HORIZONTAL, CellType.WALL_B_CROSS}
        elif col == 0:
            possible_types = {CellType.WALL_VERTICAL, CellType.WALL_L_CROSS}
        elif col == self.width - 1:
            possible_types = {CellType.WALL_VERTICAL, CellType.WALL_R_CROSS}
        cell.possible_types &= possible_types

    def generate_maze(self):
        """Generate the maze using Wave Function Collapse algorithm"""
        self.grid = [[Cell() for _ in range(self.width)]
                     for _ in range(self.height)]
        for row in range(self.height):
            for col in range(self.width):
                if row == 0 or row == self.height - 1 or col == 0 or col == self.width - 1:
                    self.propagate_border(row, col)
                    self.propagate_constraints

        self.grid[1][1] = Cell(CellType.START)
        self.grid[-2][-2] = Cell(CellType.FINISH)

        while not self.is_fully_collapsed():
            min_cell = self.find_min_entropy_cell()
            if min_cell:
                r, c = min_cell
                cell = self.grid[r][c]
                if self.debug:
                    self.print_maze(r, c)
                    print(f'min entropy cell: {min_cell}', cell.possible_types)
                # cell.collapse(random.choice(list(cell.possible_types)))
                cell.collapse_by_frequency()
                self.propagate_constraints(r, c)
            else:
                self.collapse_random_cell()
            if self.debug:
                print('------')

    def find_start_and_finish(self) -> tuple[Point, Point]:
        start = None
        finish = None

        for row in range(self.height):
            for col in range(self.width):
                cell = self.grid[row][col]
                if cell.cell_type == CellType.START:
                    start = (row, col)
                if cell.cell_type == CellType.FINISH:
                    finish = (row, col)

        if start is None or finish is None:
            raise Exception('Start or finish not found')
        return start, finish

    def print_maze(self, hr: int = -1, hc: int = -1):
        """Print the generated maze with colors and improved visualization."""
        max_entropy = len(CellType)

        # Define colors for different cell types
        colors = {
            CellType.WALL_HORIZONTAL: Fore.WHITE,
            CellType.WALL_VERTICAL: Fore.WHITE,
            CellType.WALL_CORNER_TL: Fore.WHITE,
            CellType.WALL_CORNER_TR: Fore.WHITE,
            CellType.WALL_CORNER_BL: Fore.WHITE,
            CellType.WALL_CORNER_BR: Fore.WHITE,
            CellType.WALL_T_CROSS: Fore.WHITE,
            CellType.WALL_B_CROSS: Fore.WHITE,
            CellType.WALL_L_CROSS: Fore.WHITE,
            CellType.WALL_R_CROSS: Fore.WHITE,
            CellType.WALL_CROSS: Fore.WHITE,
            CellType.START: Fore.GREEN,
            CellType.FINISH: Fore.RED,
            CellType.FLOOR: Fore.BLUE,
            CellType.SWAMP_LITE: Fore.YELLOW,
            CellType.SWAMP_MEDIUM: Fore.YELLOW,
            CellType.SWAMP_HEAVY: Fore.YELLOW,
        }

        for r, row in enumerate(self.grid):
            for c, cell in enumerate(row):
                if cell.in_path:
                    # Highlight solution path
                    print(f"{Fore.CYAN}●{Style.RESET_ALL}", end='')
                elif cell.visited:
                    # Show visited cells during pathfinding
                    print(f"{Fore.MAGENTA}○{Style.RESET_ALL}", end='')
                elif self.debug and (r == hr and c == hc):
                    # Highlight current cell in debug mode
                    print(f"\033[5m{cell.cell_type.graphic}\033[0m", end='')
                elif not cell.is_collapsed():
                    # Show entropy gradient for uncollapsed cells
                    entropy = len(cell.possible_types)
                    gray_level = int(30 + (entropy / max_entropy) * 7)
                    gray_level = min(max(gray_level, 30), 37)
                    print(
                        f"\033[{gray_level}m{cell.cell_type.graphic}\033[0m", end='')
                else:
                    # Normal cell display with color
                    color = colors.get(cell.cell_type, Fore.WHITE)
                    print(
                        f"{color}{cell.cell_type.graphic}{Style.RESET_ALL}", end='')
            print()  # Newline after each row

    def mark_visited(self, point: Point):
        """Mark a cell as visited during pathfinding"""
        row, col = point
        cell = self.grid[row][col]
        cell.visited = True

    def mark_path(self, point: Point):
        """Mark a cell as part of the solution path"""
        row, col = point
        cell = self.grid[row][col]
        cell.in_path = True

    def clear_marks(self):
        """Clear all visualization marks"""
        for row in self.grid:
            for cell in row:
                cell.visited = False
                cell.in_path = False
                cell.discarded = False
