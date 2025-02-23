use super::{MazeGenerationAlgorithm, Point, MOVEMENTS};
use crate::maze::{Maze, MazeCell};
use rand::prelude::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

#[derive(Default)]
pub struct WFC;

impl WFC {
    pub fn new() -> Self {
        WFC
    }

    fn get_valid_neighbors(&self, maze: &Maze, x: usize, y: usize) -> Vec<Point> {
        let mut neighbors = Vec::new();

        for (dx, dy) in MOVEMENTS {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;

            if maze.is_valid_move(new_x, new_y) {
                neighbors.push(Point {
                    x: new_x as usize,
                    y: new_y as usize,
                });
            }
        }
        neighbors
    }

    fn calculate_entropy(
        &self,
        point: Point,
        maze: &Maze,
        collapsed: &HashMap<Point, bool>,
    ) -> usize {
        if collapsed.contains_key(&point) {
            return 0;
        }

        // Calculate entropy based on neighboring constraints
        let neighbors = self.get_valid_neighbors(maze, point.x, point.y);
        let path_neighbors = neighbors
            .iter()
            .filter(|p| maze.get_cell(p.x, p.y) == MazeCell::Path)
            .count();

        // Favor paths over walls
        if path_neighbors < 3 {
            2 // Can be either path or wall
        } else {
            3 // Should be wall to avoid loops
        }
    }

    fn _collapse_cell(
        &self,
        point: Point,
        maze: &Maze,
        _collapsed: &HashMap<Point, bool>,
        rng: &mut impl Rng,
    ) -> MazeCell {
        let neighbors = self.get_valid_neighbors(maze, point.x, point.y);
        let path_neighbors = neighbors
            .iter()
            .filter(|p| maze.get_cell(p.x, p.y) == MazeCell::Path)
            .count();

        // Enforce maze-like structure
        if path_neighbors >= 2 {
            MazeCell::Wall
        } else {
            // Randomly choose between path and wall, with higher probability of walls
            if rng.gen_bool(0.3) {
                MazeCell::Path
            } else {
                MazeCell::Wall
            }
        }
    }
}

impl MazeGenerationAlgorithm for WFC {
    fn generate(
        &mut self,
        width: usize,
        height: usize,
        start_x: usize,
        start_y: usize,
    ) -> Option<Maze> {
        let mut maze = Maze::new(width, height);
        let mut rng = rand::thread_rng();
        let mut collapsed: HashMap<Point, bool> = HashMap::new();

        // Create borders
        for x in 0..width {
            maze.set_cell(x, 0, MazeCell::Wall);
            maze.set_cell(x, height - 1, MazeCell::Wall);
            collapsed.insert(Point { x, y: 0 }, true);
            collapsed.insert(Point { x, y: height - 1 }, true);
        }
        for y in 0..height {
            maze.set_cell(0, y, MazeCell::Wall);
            maze.set_cell(width - 1, y, MazeCell::Wall);
            collapsed.insert(Point { x: 0, y }, true);
            collapsed.insert(Point { x: width - 1, y }, true);
        }

        // Set initial path at start point and mark as entrance
        let start = Point {
            x: start_x,
            y: start_y,
        };
        maze.set_cell(start_x, start_y, MazeCell::Entrance);
        collapsed.insert(start, true);

        // Create initial paths around start point to ensure connectivity
        let mut path_count = 0;
        for (dx, dy) in MOVEMENTS {
            let new_x = (start_x as i32 + dx) as usize;
            let new_y = (start_y as i32 + dy) as usize;
            if maze.is_valid_move(new_x as i32, new_y as i32)
                && !self.is_border(&maze, new_x, new_y)
                && path_count < 2
            // Limit to 2 initial paths
            {
                maze.set_cell(new_x, new_y, MazeCell::Path);
                collapsed.insert(Point { x: new_x, y: new_y }, true);
                path_count += 1;
            }
        }

        let max_iterations = width * height * 2;
        let mut iterations = 0;

        // Main wave function collapse loop
        while collapsed.len() < width * height && iterations < max_iterations {
            iterations += 1;

            // Find cell with minimum entropy
            let mut min_entropy = usize::MAX;
            let mut min_point = None;

            for y in 0..height {
                for x in 0..width {
                    let point = Point { x, y };
                    if !collapsed.contains_key(&point) {
                        let entropy = self.calculate_entropy(point, &maze, &collapsed);
                        if entropy > 0 && entropy < min_entropy {
                            min_entropy = entropy;
                            min_point = Some(point);
                        }
                    }
                }
            }

            if let Some(point) = min_point {
                let neighbors = self.get_valid_neighbors(&maze, point.x, point.y);
                let path_neighbors = neighbors
                    .iter()
                    .filter(|p| maze.get_cell(p.x, p.y) == MazeCell::Path)
                    .count();

                // Increase path probability significantly
                let path_probability = match path_neighbors {
                    0 => 0.7, // High probability if no path neighbors
                    1 => 0.8, // Higher probability with one path neighbor
                    2 => 0.4, // Medium probability with two path neighbors
                    _ => 0.2, // Lower probability with more path neighbors
                };

                let cell_type = if rng.gen_bool(path_probability) {
                    MazeCell::Path
                } else {
                    MazeCell::Wall
                };

                maze.set_cell(point.x, point.y, cell_type);
                collapsed.insert(point, true);

                // Improve connectivity logic
                if cell_type == MazeCell::Path && path_neighbors == 0 {
                    let mut shuffled_neighbors = neighbors.clone();
                    shuffled_neighbors.shuffle(&mut rng);

                    for neighbor in shuffled_neighbors {
                        if maze.get_cell(neighbor.x, neighbor.y) == MazeCell::Wall
                            && !self.is_border(&maze, neighbor.x, neighbor.y)
                        {
                            maze.set_cell(neighbor.x, neighbor.y, MazeCell::Path);
                            collapsed.insert(neighbor, true);
                            break;
                        }
                    }
                }
            } else {
                // If we can't find a cell to collapse, mark remaining as paths
                for y in 1..height - 1 {
                    for x in 1..width - 1 {
                        let point = Point { x, y };
                        if !collapsed.contains_key(&point) {
                            maze.set_cell(x, y, MazeCell::Path);
                            collapsed.insert(point, true);
                        }
                    }
                }
                break;
            }
        }

        // Improved exit point selection
        let mut best_exit = None;
        let mut max_distance = 0.0;
        let mut best_path_count = 0;

        // Check all border points
        for x in 0..width {
            for y in [0, height - 1] {
                if self.has_path_neighbor(&maze, x, y) {
                    let distance = ((x as i32 - start_x as i32).pow(2)
                        + (y as i32 - start_y as i32).pow(2))
                        as f64;
                    let path_count = self.count_path_neighbors(&maze, x, y);

                    // Prioritize points with more path neighbors
                    let score = distance * (path_count as f64 + 1.0);
                    if score > max_distance {
                        max_distance = score;
                        best_exit = Some((x, y));
                        best_path_count = path_count;
                    }
                }
            }
        }
        for y in 1..height - 1 {
            for x in [0, width - 1] {
                if self.has_path_neighbor(&maze, x, y) {
                    let distance = ((x as i32 - start_x as i32).pow(2)
                        + (y as i32 - start_y as i32).pow(2))
                        as f64;
                    let path_count = self.count_path_neighbors(&maze, x, y);

                    let score = distance * (path_count as f64 + 1.0);
                    if score > max_distance {
                        max_distance = score;
                        best_exit = Some((x, y));
                        best_path_count = path_count;
                    }
                }
            }
        }

        // If no good exit found or path count is too low, create a new path
        if best_path_count < 2 {
            if let Some((x, y)) = best_exit {
                // Create additional paths around the exit
                for (dx, dy) in MOVEMENTS {
                    let new_x = (x as i32 + dx) as usize;
                    let new_y = (y as i32 + dy) as usize;
                    if maze.is_valid_move(new_x as i32, new_y as i32)
                        && !self.is_border(&maze, new_x, new_y)
                    {
                        maze.set_cell(new_x, new_y, MazeCell::Path);
                    }
                }
            }
        }

        let exit_point = best_exit.unwrap_or_else(|| {
            // If no exit found, create a path to a border
            let border_points = vec![
                (width - 2, height - 1),
                (width - 1, height - 2),
                (1, 0),
                (0, 1),
            ];
            let chosen = border_points.choose(&mut rng).unwrap();
            maze.set_cell(chosen.0, chosen.1, MazeCell::Path);
            *chosen
        });

        maze.set_cell(exit_point.0, exit_point.1, MazeCell::Exit);

        // After setting exit point, ensure there's a path from entrance to exit
        if let Some(path) = self.find_path_to_entrance(&mut maze, exit_point, (start_x, start_y)) {
            for &(x, y) in &path {
                if maze.get_cell(x, y) != MazeCell::Entrance
                    && maze.get_cell(x, y) != MazeCell::Exit
                {
                    maze.set_cell(x, y, MazeCell::Path);
                }
            }
        }

        maze.backup();
        Some(maze)
    }
}

impl WFC {
    // Add helper methods
    fn is_border(&self, maze: &Maze, x: usize, y: usize) -> bool {
        x == 0 || y == 0 || x == maze.width - 1 || y == maze.height - 1
    }

    fn has_path_neighbor(&self, maze: &Maze, x: usize, y: usize) -> bool {
        for (dx, dy) in MOVEMENTS {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            if maze.is_valid_move(new_x, new_y) {
                let cell = maze.get_cell(new_x as usize, new_y as usize);
                if cell == MazeCell::Path {
                    return true;
                }
            }
        }
        false
    }

    fn count_path_neighbors(&self, maze: &Maze, x: usize, y: usize) -> usize {
        MOVEMENTS
            .iter()
            .filter(|(dx, dy)| {
                let new_x = x as i32 + dx;
                let new_y = y as i32 + dy;
                maze.is_valid_move(new_x, new_y)
                    && maze.get_cell(new_x as usize, new_y as usize) == MazeCell::Path
            })
            .count()
    }

    // Add helper method to find path to entrance
    fn find_path_to_entrance(
        &self,
        maze: &mut Maze,
        start: (usize, usize),
        end: (usize, usize),
    ) -> Option<Vec<(usize, usize)>> {
        use std::collections::VecDeque;

        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();

        queue.push_back(start);
        visited.insert(start, start);

        while let Some(current) = queue.pop_front() {
            if current == end {
                // Reconstruct path
                let mut path = vec![current];
                let mut pos = current;
                while pos != start {
                    pos = visited[&pos];
                    path.push(pos);
                }
                path.reverse();
                return Some(path);
            }

            for (dx, dy) in MOVEMENTS {
                let new_x = current.0 as i32 + dx;
                let new_y = current.1 as i32 + dy;

                if maze.is_valid_move(new_x, new_y) {
                    let next = (new_x as usize, new_y as usize);
                    let cell = maze.get_cell(next.0, next.1);

                    if (cell == MazeCell::Path
                        || cell == MazeCell::Entrance
                        || cell == MazeCell::Exit)
                        && !visited.contains_key(&next)
                    {
                        visited.insert(next, current);
                        queue.push_back(next);
                    }
                }
            }
        }

        None
    }
}
