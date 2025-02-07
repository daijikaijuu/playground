from enum import Enum


class CellType(Enum):
    FLOOR = 0
    WALL = 1
    WALL_CORNER_TL = 2
    WALL_CORNER_TR = 3
    WALL_CORNER_BL = 4
    WALL_CORNER_BR = 5
    WALL_CORNER_TL_CORNER_TR = 6
    WALL_CORNER_BL_CORNER_BR = 7
    WALL_CORNER_TL_CORNER_BL = 8
    WALL_CORNER_TR_CORNER_BR = 9


class Cell:
    cell_type: CellType

    def __init__(self, cell_type: CellType):
        self.cell_type = cell_type
