import maze_lib


def main() -> None:
    maze = maze_lib.Maze(40, 10)
    maze.generate_maze()
    maze.print_maze()


if __name__ == "__main__":
    main()
