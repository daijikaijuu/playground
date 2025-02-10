from maze_lib.path_solving.path_solving import PathSolving


class DFS(PathSolving):
    def find_path(self):
        stack = [self.start]
        visited = set()

        while stack:
            current = stack.pop()
            if current in visited:
                continue

            visited.add(current)

            if current == self.finish:
                return True
