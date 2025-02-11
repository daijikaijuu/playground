from maze_lib.path_solving.path_solving import PathSolving


class BFS(PathSolving):
    def find_path(self) -> bool:
        stack = [self.start]
        self.visited = set()

        while stack:
            current = stack.pop()
            if current in self.visited:
                continue

            self.visited.add(current)

            self.print_step()

            if current == self.finish:
                return True

            (row, col), _ = current
            neighbors = self.get_neighbors(col, row, valid=True)
            for neighbor in neighbors:
                if neighbor not in self.visited:
                    # print(neighbor)
                    stack.append(neighbor)

    def print_step(self):
        print('\033[0J\033[H')
        for r, row in enumerate(self.maze.grid):
            for c, cell in enumerate(row):
                if ((r, c), cell) in self.visited:
                    print('X', end='')
                else:
                    print(cell.cell_type.graphic, end='')
            print()
        print('-----------')
