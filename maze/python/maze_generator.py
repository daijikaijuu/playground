from maze_lib import BFS, Maze
from maze_lib.cell import Cell, CellType
import json
from typing import Optional


def generate_maze(width: int, height: int, ensure_solvable: bool = True) \
        -> Maze:
    while True:
        try:
            maze = Maze(width, height, False)
            if ensure_solvable:
                solvable = False
                print('trying to generate solvable maze...')
                while not solvable:
                    maze.generate_maze()
                    bfs = BFS(maze, step_delay=0.0)
                    solvable = bfs.find_path()
                return maze
            else:
                maze.generate_maze()
                return maze
        except ValueError:
            print("Failed to generate maze. Retrying...")

def save_maze(maze: Maze, filename: str) -> None:
    """Save maze to a JSON file"""
    maze_data = {
        'width': maze.width,
        'height': maze.height,
        'grid': [[cell.cell_type.name if cell.cell_type else None 
                 for cell in row] for row in maze.grid]
    }
    with open(filename, 'w') as f:
        json.dump(maze_data, f)

def load_maze(filename: str) -> Optional[Maze]:
    """Load maze from a JSON file"""
    try:
        with open(filename, 'r') as f:
            data = json.load(f)
            maze = Maze(data['width'], data['height'])
            maze.grid = [[Cell(CellType[cell_type] if cell_type else None) 
                         for cell_type in row] for row in data['grid']]
            return maze
    except (FileNotFoundError, json.JSONDecodeError, KeyError):
        return None
