import argparse

from maze_generator import generate_maze
from maze_lib import BFS, DFS, AStar, Dijkstra


def main(args: argparse.Namespace) -> None:
    maze = generate_maze(40, 20)
    alg = None
    match args.algorithm:
        case 'BFS':
            alg = BFS(maze, step_delay=args.timeout, debug=True)
        case 'DFS':
            alg = DFS(maze, step_delay=args.timeout, debug=True)
        case 'dijkstra':
            alg = Dijkstra(maze, step_delay=args.timeout, debug=True)
        case 'astar':
            alg = AStar(maze, step_delay=args.timeout, debug=True)
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
    parser.add_argument('-a', '--algorithm', default='BFS',
                        dest='algorithm',
                        choices=['BFS', 'DFS', 'dijkstra', 'astar'],
                        help='Pathfinding algorithm to use')
    return parser.parse_args()


if __name__ == "__main__":
    args = parse_args()
    main(args)
