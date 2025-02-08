from enum import Enum


class CellType(Enum):
    FLOOR = (0, ' ')
    WALL_HORIZONTAL = (1, '─')
    WALL_VERTICAL = (2, '│')
    WALL_CORNER_TL = (3, '┌')
    WALL_CORNER_TR = (4, '┐')
    WALL_CORNER_BL = (5, '└')
    WALL_CORNER_BR = (6, '┘')
    # WALL_CROSS = (7, '┼')
    WALL_T_CROSS = (8, '┬')
    # WALL_B_CROSS = (9, '┴')
    # WALL_L_CROSS = (10, '├')
    # WALL_R_CROSS = (11, '┤')

    def __init__(self, value: int, graphic: str):
        self._value_ = value
        self.graphic = graphic


class Cell:
    cell_type: CellType
    possible_types: set[CellType]

    def __init__(self, cell_type: CellType = None):
        self.cell_type = cell_type
        self.possible_types = set(CellType)

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
            self.cell_type = self.possible_types[0]
        else:
            raise ValueError("Multiple possible types remaining")

    def is_collapsed(self) -> bool:
        """Check if the cell has been collapsed"""
        return len(self.possible_types) == 1 and {self.cell_type} == self.possible_types
