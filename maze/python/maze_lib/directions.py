from enum import Enum


class Directions(Enum):
    LEFT = (0, -1)
    RIGHT = (0, 1)
    DOWN = (1, 0)
    UP = (-1, 0)
    TOP_LEFT = (-1, -1)
    TOP_RIGHT = (-1, 1)
    BOTTOM_LEFT = (1, -1)
    BOTTOM_RIGHT = (1, 1)

    @property
    def delta(self) -> tuple[int, int]:
        """Return the row and column deltas for the direction."""
        return self.value

    @staticmethod
    def get_directions(diagonal: bool = False) -> list[tuple[int, int]]:
        if diagonal:
            return [dir.value for dir in Directions]
        else:
            return [dir.value for dir in Directions
                    if dir not in {
                        Directions.TOP_LEFT,
                        Directions.TOP_RIGHT,
                        Directions.BOTTOM_LEFT,
                        Directions.BOTTOM_RIGHT}]

    @staticmethod
    def calculate_direction(from_point: tuple[int, int], to_point: tuple[int, int]):
        """
        Calculate the direction from one point to another using pattern matching.

        Args:
            from_point (Tuple[int, int]): The starting point (row, col).
            to_point (Tuple[int, int]): The target point (row, col).

        Returns:
            Directions: The direction from the starting point to the target point.
        """
        dr = to_point[0] - from_point[0]
        dc = to_point[1] - from_point[1]

        match (dr, dc):
            case (0, -1):
                return Directions.LEFT
            case (0, 1):
                return Directions.RIGHT
            case (-1, 0):
                return Directions.UP
            case (1, 0):
                return Directions.DOWN
            case (-1, -1):
                return Directions.TOP_LEFT
            case (-1, 1):
                return Directions.TOP_RIGHT
            case (1, -1):
                return Directions.BOTTOM_LEFT
            case (1, 1):
                return Directions.BOTTOM_RIGHT
            case _:
                raise ValueError(
                    f"Invalid direction from {from_point} to {to_point}")
