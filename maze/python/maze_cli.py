import argparse
import sys
from maze_generator import generate_maze, save_maze_to_json, load_maze_from_json
from maze_lib import BFS, DFS, AStar, Dijkstra, Backtracking


def main(args: argparse.Namespace) -> None:
    # Load maze if specified
    if args.load:
        maze = load_maze_from_json(args.load)
        if maze is None:
            print(f"Error: Could not load maze from {args.load}")
            sys.exit(1)
    else:
        maze = generate_maze(args.width, args.height, ensure_solvable=True)
    
    # Save maze if specified
    if args.save:
        save_maze_to_json(maze, args.save)
        print(f"Maze saved to {args.save}")
        if not args.algorithm:
            return

    if not args.algorithm:
        # Just print the maze if no algorithm specified
        maze.print_maze()
        return

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
        case 'backtracking':
            alg = Backtracking(maze, step_delay=args.timeout, debug=True)
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
    parser.add_argument('-a', '--algorithm', default=None,
                        dest='algorithm',
                        choices=['BFS', 'DFS', 'dijkstra', 'astar', 'backtracking'],
                        help='Pathfinding algorithm to use')
    parser.add_argument('-s', '--save',
                        help='Save maze to specified file')
    parser.add_argument('-l', '--load',
                        help='Load maze from specified file')
    return parser.parse_args()


if __name__ == "__main__":
    args = parse_args()
    main(args)
