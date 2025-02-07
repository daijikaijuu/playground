from enum import Enum


class Directions(Enum):
    UP = (0, -1)
    DOWN = (0, 1)
    LEFT = (1, 0)
    RIGHT = (-1, 0)

    @property
    def delta(self) -> tuple[int, int]:
        """Return the row and column deltas for the direction."""
        return self.value

    @staticmethod
    def get_directions() -> 'Directions':
        return [dir.value for dir in Directions]

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
                return Directions.UP
            case (0, 1):
                return Directions.DOWN
            case (-1, 0):
                return Directions.LEFT
            case (1, 0):
                return Directions.RIGHT
            case _:
                raise ValueError(
                    f"Invalid direction from {from_point} to {to_point}")
