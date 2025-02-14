import tkinter as tk
from tkinter import ttk, filedialog

from maze_generator import generate_maze, save_maze_to_json, load_maze_from_json
from maze_lib import CellType, Maze, BFS, DFS, AStar, Dijkstra

CELL_SIZE = 20
MAZE_WIDTH = 40
MAZE_HEIGHT = 40


def draw_maze(canvas, maze: Maze):
    canvas.delete('all')
    for row in range(maze.height):
        for col in range(maze.width):
            cell = maze.grid[row][col]
            outline = 'gray'
            
            # Base color based on cell type
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
            
            # Overlay visualization colors
            if cell.in_path:
                fill = 'yellow'  # Path cells
            elif cell.visited and cell.cell_type not in [CellType.START, CellType.FINISH]:
                fill = '#FFB6C1'  # Light pink for visited cells
                
            canvas.create_rectangle(col*CELL_SIZE, row*CELL_SIZE,
                                  col*CELL_SIZE+CELL_SIZE, row*CELL_SIZE+CELL_SIZE,
                                  fill=fill, outline=outline)


def update_canvas(root, canvas, maze: Maze):
    draw_maze(canvas, maze)
    root.after(16, update_canvas, root, canvas, maze)  # Update more frequently (60 FPS)


def run_algorithm(maze: Maze, algorithm_name: str, step_delay: float = 0.1):
    # Clear previous visualization
    maze.clear_marks()
    
    alg = None
    match algorithm_name:
        case 'BFS':
            alg = BFS(maze, step_delay=step_delay, debug=False)
        case 'DFS':
            alg = DFS(maze, step_delay=step_delay, debug=False)
        case 'Dijkstra':
            alg = Dijkstra(maze, step_delay=step_delay, debug=False)
        case 'A*':
            alg = AStar(maze, step_delay=step_delay, debug=False)
    
    if alg:
        def solve_step():
            try:
                if alg.step():  # If step was successful, schedule next step
                    root.after(int(step_delay * 1000), solve_step)
                else:
                    print("Unsolvable")
            except StopIteration:
                pass  # Algorithm finished
        
        solve_step()  # Start the solving process


def main():
    maze = generate_maze(MAZE_WIDTH, MAZE_HEIGHT, ensure_solvable=True)

    global root  # Make root accessible to run_algorithm
    root = tk.Tk()
    root.title('Maze crawler. Python')
    
    # Create main frame
    main_frame = ttk.Frame(root, padding="5")
    main_frame.grid(row=0, column=0, sticky=(tk.W, tk.E, tk.N, tk.S))
    
    # Create canvas
    canvas = tk.Canvas(main_frame, width=CELL_SIZE * MAZE_WIDTH,
                      height=CELL_SIZE * MAZE_HEIGHT, bg='white')
    canvas.grid(row=0, column=0, padx=5, pady=5)

    # Create control frame
    control_frame = ttk.Frame(main_frame)
    control_frame.grid(row=0, column=1, padx=5, pady=5, sticky=(tk.N, tk.S))
    
    # Algorithm selection
    ttk.Label(control_frame, text="Algorithm:").pack(pady=5)
    algorithm_var = tk.StringVar(value="BFS")
    algorithms = ['BFS', 'DFS', 'Dijkstra', 'A*']
    algorithm_combo = ttk.Combobox(control_frame, textvariable=algorithm_var, 
                                 values=algorithms, state='readonly')
    algorithm_combo.pack(pady=5)
    
    # Step delay control
    ttk.Label(control_frame, text="Step Delay (s):").pack(pady=5)
    delay_var = tk.StringVar(value="0.1")
    delay_entry = ttk.Entry(control_frame, textvariable=delay_var, width=10)
    delay_entry.pack(pady=5)
    
    # Run button
    def on_run():
        try:
            delay = float(delay_var.get())
            run_algorithm(maze, algorithm_var.get(), delay)
        except ValueError:
            print("Invalid delay value")
    
    ttk.Button(control_frame, text="Run Algorithm", 
               command=on_run).pack(pady=10)
    
    # Save/Load buttons
    def save_maze():
        file_path = filedialog.asksaveasfilename(
            defaultextension=".maze",
            filetypes=[("Maze files", "*.maze"), ("All files", "*.*")]
        )
        if file_path:
            save_maze_to_json(maze, file_path)  # Using the function from maze_generator
    
    def load_maze():
        file_path = filedialog.askopenfilename(
            filetypes=[("Maze files", "*.maze"), ("All files", "*.*")]
        )
        if file_path:
            nonlocal maze
            loaded_maze = load_maze_from_json(file_path)  # Using the function from maze_generator
            if loaded_maze:
                maze = loaded_maze
                maze.clear_marks()  # Clear any visualization marks
                draw_maze(canvas, maze)  # Force immediate redraw
            else:
                print("Failed to load maze")
    
    ttk.Button(control_frame, text="Save Maze", 
               command=save_maze).pack(pady=5)
    ttk.Button(control_frame, text="Load Maze", 
               command=load_maze).pack(pady=5)

    # Add separator between save/load and new maze buttons
    ttk.Separator(control_frame, orient='horizontal').pack(fill='x', pady=10)

    # New maze button
    def on_new_maze():
        nonlocal maze
        maze = generate_maze(MAZE_WIDTH, MAZE_HEIGHT, ensure_solvable=True)
        draw_maze(canvas, maze)  # Force immediate redraw
    
    ttk.Button(control_frame, text="New Maze", 
               command=on_new_maze).pack(pady=10)

    update_canvas(root, canvas, maze)
    root.mainloop()


if __name__ == "__main__":
    main()
