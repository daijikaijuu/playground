import argparse
import os
import sys
from gettext import gettext as _

from colorama import Fore, Style, init

from maze_generator import (generate_maze, load_maze_from_json,
                            save_maze_to_json)
from maze_lib import BFS, DFS, AStar, Backtracking, BellmanFord, Dijkstra

# Initialize colorama
init()

# Enable VT100 Escape Sequence for WINDOWS 10 Ver. 1607
os.system('')


def main(args: argparse.Namespace) -> None:
    # Load maze if specified
    if args.load:
        print(_(f"{Fore.CYAN}Loading maze from {args.load}...{Style.RESET_ALL}"))
        maze = load_maze_from_json(args.load)
        if maze is None:
            print(
                _(f"{Fore.RED}Error: Could not load maze from {args.load}{Style.RESET_ALL}"))
            sys.exit(1)
        print(_(f"{Fore.GREEN}Maze loaded successfully!{Style.RESET_ALL}\n"))
    else:
        print(
            f"{Fore.CYAN}Generating {args.width}x{args.height} maze...{Style.RESET_ALL}")
        maze = generate_maze(args.width, args.height, ensure_solvable=True)
        print(f"{Fore.GREEN}Maze generated successfully!{Style.RESET_ALL}\n")

    # Save maze if specified
    if args.save:
        print(f"{Fore.CYAN}Saving maze to {args.save}...{Style.RESET_ALL}")
        save_maze_to_json(maze, args.save)
        print(f"{Fore.GREEN}Maze saved successfully!{Style.RESET_ALL}\n")
        if not args.algorithm:
            return

    if not args.algorithm:
        # Just print the maze if no algorithm specified
        print(f"{Fore.YELLOW}Maze layout:{Style.RESET_ALL}")
        maze.print_maze()
        return

    alg = None
    algorithm_name = args.algorithm.upper()
    print(f"{Fore.CYAN}Solving maze using {algorithm_name} algorithm...{Style.RESET_ALL}")

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
        case 'bellman-ford':
            alg = BellmanFord(maze, step_delay=args.timeout, debug=True)

    if not alg.find_path():
        print(f"{Fore.RED}Maze is unsolvable!{Style.RESET_ALL}")
    else:
        print(f"\n{Fore.GREEN}Path found successfully!{Style.RESET_ALL}")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=f"{Fore.CYAN}Maze Generator and Solver CLI{Style.RESET_ALL}",
        formatter_class=argparse.RawDescriptionHelpFormatter
    )
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
                        choices=['BFS', 'DFS', 'dijkstra', 'astar',
                                 'backtracking', 'bellman-ford'],
                        help='Pathfinding algorithm to use')
    parser.add_argument('-s', '--save',
                        help='Save maze to specified file')
    parser.add_argument('-l', '--load',
                        help='Load maze from specified file')
    return parser.parse_args()


if __name__ == "__main__":
    args = parse_args()
    main(args)
