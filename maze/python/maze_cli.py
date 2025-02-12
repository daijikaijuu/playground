import argparse

from maze_generator import generate_maze
from maze_lib import BFS, DFS


def main(args: argparse.Namespace) -> None:
    maze = generate_maze(40, 20)
    alg = BFS(maze, step_delay=args.timeout, debug=True)
    if not alg.find_path():
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
