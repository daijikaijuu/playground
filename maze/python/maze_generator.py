from maze_lib import BFS, Maze


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
