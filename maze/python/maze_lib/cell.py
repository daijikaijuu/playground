import random
from enum import Enum


class CellType(Enum):
    FLOOR = (0, ' ', 0.98, 1.0)
    WALL_HORIZONTAL = (1, '─', 0.3, 0.0)
    WALL_VERTICAL = (2, '│', 0.05, 0.0)
    WALL_CORNER_TL = (3, '┌', 0.1, 0.0)
    WALL_CORNER_TR = (4, '┐', 0.1, 0.0)
    WALL_CORNER_BL = (5, '└', 0.1, 0.0)
    WALL_CORNER_BR = (6, '┘', 0.1, 0.0)
    WALL_CROSS = (7, '┼', 0.1, 0.0)
    WALL_T_CROSS = (8, '┬', 0.1, 0.0)
    WALL_B_CROSS = (9, '┴', 0.1, 0.0)
    WALL_L_CROSS = (10, '├', 0.1, 0.0)
    WALL_R_CROSS = (11, '┤', 0.1, 0.0)

    SWAMP_LITE = (20, '░', 0.05, 0.8)
    SWAMP_MEDIUM = (21, '▒', 0.02, 0.4)
    SWAMP_HEAVY = (22, '▓', 0.005, 0.2)

    START = (40, 's', 0.0, 1.0)
    FINISH = (41, 'f', 0.0, 1.0)

    def __init__(self, value: int, graphic: str, frequency: float, walkable: float):
        self._value_ = value
        self.graphic = graphic
        self.frequency = frequency
        self.walkable = walkable


class Cell:
    cell_type: CellType
    possible_types: set[CellType]
    visited: bool
    in_path: bool
    discarded: bool

    def __init__(self, cell_type: CellType | None = None):
        self.cell_type = cell_type
        if cell_type is None:
            self.cell_type = CellType.FLOOR
            self.possible_types = set(CellType) ^ {
                CellType.START, CellType.FINISH}
        else:
            self.cell_type = cell_type
            self.possible_types = {cell_type}

        self.visited = False  # For pathfinding visualization
        self.in_path = False  # For showing the solution path
        self.discarded = False  # For showing backtracked paths

    def collapse(self, cell_type: CellType | None = None):
        """Collapse the cell to a specific type, or to the single remaining type"""
        if cell_type is None:
            self.collapse_single()
        else:
            self.cell_type = cell_type
            self.possible_types = {cell_type}

    def collapse_single(self):
        """Collapse the cell to the single remaining possible type"""
        if len(self.possible_types) == 1:
            self.cell_type = list(self.possible_types)[0]
        else:
            raise ValueError("Multiple possible types remaining")

    def is_collapsed(self) -> bool:
        """Check if the cell has been collapsed"""
        return len(self.possible_types) == 1 and {self.cell_type} == self.possible_types

    def collapse_by_frequency(self):
        if len(self.possible_types) == 1:
            self.cell_type = list(self.possible_types)[0]
        else:
            filtered_frequency = {
                cell_type: cell_type.frequency
                for cell_type in self.possible_types}
            total_frequency = sum(filtered_frequency.values())

            nomalized_frequency = {
                cell_type: freq / total_frequency
                for cell_type, freq in filtered_frequency.items()}
            self.cell_type = random.choices(
                list(nomalized_frequency.keys()),
                weights=list(nomalized_frequency.values()), k=1)[0]
            self.possible_types = {self.cell_type}
