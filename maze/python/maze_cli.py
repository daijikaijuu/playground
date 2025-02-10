import maze_lib


def generate_maze(width: int, height: int):
    while True:
        try:
            maze = maze_lib.Maze(width, height, False)
            maze.generate_maze()
            maze.print_maze()
            return
        except ValueError:
            print("Failed to generate maze. Retrying...")


def main() -> None:
    generate_maze(40, 20)


if __name__ == "__main__":
    main()
