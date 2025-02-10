import random
from enum import Enum


class CellType(Enum):
    FLOOR = (0, ' ', 0.8)
    WALL_HORIZONTAL = (1, '─', 0.3)
    WALL_VERTICAL = (2, '│', 0.05)
    WALL_CORNER_TL = (3, '┌', 0.1)
    WALL_CORNER_TR = (4, '┐', 0.1)
    WALL_CORNER_BL = (5, '└', 0.1)
    WALL_CORNER_BR = (6, '┘', 0.1)
    WALL_CROSS = (7, '┼', 0.1)
    WALL_T_CROSS = (8, '┬', 0.1)
    WALL_B_CROSS = (9, '┴', 0.1)
    WALL_L_CROSS = (10, '├', 0.1)
    WALL_R_CROSS = (11, '┤', 0.1)

    SWAMP_LITE = (20, '░', 0.05)
    SWAMP_MEDIUM = (21, '▒', 0.02)
    SWAMP_HEAVY = (22, '▓', 0.005)

    START = (40, 's', 0.0)
    FINISH = (41, 'f', 0.0)

    def __init__(self, value: int, graphic: str, frequency: float):
        self._value_ = value
        self.graphic = graphic
        self.frequency = frequency


class Cell:
    cell_type: CellType
    possible_types: set[CellType]

    def __init__(self, cell_type: CellType | None = None):
        if cell_type is None:
            self.cell_type = CellType.FLOOR
            self.possible_types = set(CellType) ^ {
                CellType.START, CellType.FINISH}
        else:
            self.cell_type = cell_type
            self.possible_types = {cell_type}

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
