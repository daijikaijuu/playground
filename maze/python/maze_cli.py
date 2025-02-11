from maze_lib import BFS, Maze


def generate_maze(width: int, height: int) -> Maze:
    while True:
        try:
            maze = Maze(width, height, False)
            maze.generate_maze()
            return maze
        except ValueError:
            print("Failed to generate maze. Retrying...")


def main() -> None:
    maze = generate_maze(40, 20)
    dfs = BFS(maze)
    if not dfs.find_path():
        print("Unsolvable")


if __name__ == "__main__":
    main()
