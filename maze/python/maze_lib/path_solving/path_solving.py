from abc import ABC, abstractmethod

from maze_lib.maze import Maze
from maze_lib.types import Point


class PathSolving(ABC):
    maze: Maze
    start: Point
    finish: Point

    def __init__(self, maze: Maze):
        self.maze = maze
        self.start, self.finish = self.maze.find_start_and_finish()

    @abstractmethod
    def find_path(self):
        pass
