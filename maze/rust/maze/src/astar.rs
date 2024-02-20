use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use crate::{
    maze::{Maze, MazeCell},
    pathfinding::PathfindingAlgorithm,
    visualization::MazeVisualization,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Node {
    point: Point,
    g: u32,
    h: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // A* uses f(n) = g(n) + h(n) as the cost function
        (other.g + other.h).cmp(&(self.g + self.h))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct AStar {
    pub visualization_delay: u64,
}

impl AStar {
    pub fn new(visualization_delay: u64) -> Self {
        AStar {
            visualization_delay,
        }
    }

    fn heuristic(&self, current: &Point, goal: &Point) -> u32 {
        // Simple Manhattan distance as the heuristic
        ((current.x as i32 - goal.x as i32).abs() + (current.y as i32 - goal.y as i32).abs()) as u32
    }

    fn reconstruct_path(
        came_from: &HashMap<Point, Point>,
        mut current: Point,
        running_flag: Arc<Mutex<bool>>,
    ) -> Vec<Point> {
        let mut path = vec![current];
        while came_from.contains_key(&current) && *running_flag.lock().unwrap() {
            current = *came_from.get(&current).unwrap();
            path.push(current);
        }
        path.reverse();
        path
    }
}

impl PathfindingAlgorithm for AStar {
    fn find_path(&mut self, visualization: &mut MazeVisualization) -> bool {
        let running_flag = Arc::new(Mutex::new(true));
        let maze = Arc::new(Mutex::new(visualization.maze.clone()));

        let (sender, receiver): (Sender<Maze>, Receiver<Maze>) = channel();

        let running_flag_clone = Arc::clone(&running_flag);
        let handle = thread::spawn(move || {
            let mut maze = maze.lock().unwrap();
            let astar = AStar::new(20);

            // Find entrance and exit coordinates
            let entrance = maze.get_entrance().unwrap();
            let exit = maze.get_exit().unwrap();

            let start = Point {
                x: entrance.0,
                y: entrance.1,
            };
            let goal = Point {
                x: exit.0,
                y: exit.1,
            };

            let mut open_set = BinaryHeap::new();
            let mut came_from: HashMap<Point, Point> = HashMap::new();
            let mut g_scores: HashMap<Point, u32> = HashMap::new();

            open_set.push(Node {
                point: start,
                g: 0,
                h: astar.heuristic(&start, &goal),
            });
            g_scores.insert(start, 0);

            while let Some(current_node) = open_set.pop() {
                let current = current_node.point;

                if current == goal {
                    let path = AStar::reconstruct_path(
                        &came_from,
                        current,
                        Arc::clone(&running_flag_clone),
                    );
                    for point in path.iter().skip(1) {
                        maze.set_cell(point.x, point.y, MazeCell::FinalPath);

                        sender
                            .send(maze.clone())
                            .expect("Failed to send maze to the main thread");
                    }
                    *running_flag_clone.lock().unwrap() = false;
                }

                for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let neighbor = Point {
                        x: (current.x as i32 + dx) as usize,
                        y: (current.y as i32 + dy) as usize,
                    };

                    if !maze.is_valid_move(neighbor.x as i32, neighbor.y as i32)
                        || maze.get_cell(neighbor.x, neighbor.y) == MazeCell::Wall
                    {
                        continue;
                    }

                    let tentative_g_score = g_scores[&current] + 1;

                    if !g_scores.contains_key(&neighbor) || tentative_g_score < g_scores[&neighbor]
                    {
                        g_scores.insert(neighbor, tentative_g_score);
                        came_from.insert(neighbor, current);

                        open_set.push(Node {
                            point: neighbor,
                            g: tentative_g_score,
                            h: astar.heuristic(&neighbor, &goal),
                        });
                    }
                }
            }
        });

        while let Ok(recieved_maze) = receiver.recv() {
            if visualization.rl.window_should_close() {
                *running_flag.lock().unwrap() = false;
                return false;
            }

            // Update visualization with the new maze
            visualization.set_maze(&recieved_maze);

            visualization.visualize(self.name());

            thread::sleep(Duration::from_millis(self.visualization_delay));
        }

        // Check for potential errors
        if let Err(err) = receiver.recv() {
            eprintln!("Error receiving maze: {}", err);
        }

        handle.join().expect("Handle join error");

        false
    }

    fn name(&self) -> &str {
        "A*"
    }
}
