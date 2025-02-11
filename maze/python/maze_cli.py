import argparse

from maze_lib import BFS, Maze


def generate_maze(width: int, height: int, ensure_solvable: bool = True) -> Maze:
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


def main(args: argparse.Namespace) -> None:
    maze = generate_maze(40, 20)
    dfs = BFS(maze, step_delay=args.timeout, debug=True)
    if not dfs.find_path():
        print("Unsolvable")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("-w", "--width", type=int, default=40,
                        help="Width of the maze grid")
    parser.add_argument("-H", "--height", type=int, default=20,
                        help="Height of the maze grid")
    parser.add_argument("-d", "--debug", action="store_true",
                        help="Enable debug mode")
    parser.add_argument("-t", "--timeout", type=float, default=0.1,
                        dest='timeout',
                        help="Time delay between pathfinding steps")
    return parser.parse_args()


if __name__ == "__main__":
    args = parse_args()
    main(args)
