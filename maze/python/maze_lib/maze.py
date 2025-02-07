from .cell import Cell, CellType


class Maze:
    maze: list[list[Cell]]

    def __init__(self, width: int, height: int):
        self.maze = [[Cell(CellType.FLOOR) for _ in range(width)]
                     for _ in range(height)]
