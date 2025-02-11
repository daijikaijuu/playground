import tkinter as tk

from maze_generator import generate_maze
from maze_lib import CellType, Maze

CELL_SIZE = 20
MAZE_WIDTH = 40
MAZE_HEIGHT = 40


def draw_maze(canvas, maze: Maze):
    canvas.delete('all')
    for row in range(maze.height):
        for col in range(maze.width):
            cell = maze.grid[row][col]
            outline = 'gray'
            match cell.cell_type:
                case CellType.FLOOR:
                    fill = 'white'
                case CellType.START:
                    fill = 'red'
                case CellType.FINISH:
                    fill = 'blue'
                case CellType.SWAMP_LITE:
                    fill = '#00AA00'
                case CellType.SWAMP_MEDIUM:
                    fill = '#006600'
                case CellType.SWAMP_HEAVY:
                    fill = '#003300'
                case _:
                    fill = 'black'
            canvas.create_rectangle(col*CELL_SIZE, row*CELL_SIZE,
                                    col*CELL_SIZE+CELL_SIZE, row*CELL_SIZE+CELL_SIZE,
                                    fill=fill, outline=outline)


def update_canvas(root, canvas, maze: Maze):
    draw_maze(canvas, maze)
    root.after(100, update_canvas, root, canvas, maze)


def main():
    maze = generate_maze(MAZE_WIDTH, MAZE_HEIGHT, ensure_solvable=False)

    root = tk.Tk()
    root.title('Maze crawler. Python')
    root.geometry('800x600')
    canvas = tk.Canvas(root, width=CELL_SIZE * MAZE_WIDTH,
                       height=CELL_SIZE * MAZE_HEIGHT, bg='white')
    canvas.pack(side=tk.LEFT, fill=tk.BOTH, padx=5, pady=5)

    update_canvas(root, canvas, maze)

    root.mainloop()


if __name__ == "__main__":
    main()
