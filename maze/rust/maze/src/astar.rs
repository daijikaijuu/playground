use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    thread,
    time::Duration,
};

use crate::{maze::MazeCell, pathfinding::PathfindingAlgorithm, visualization::MazeVisualization};

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

    fn reconstruct_path(came_from: HashMap<Point, Point>, mut current: Point) -> Vec<Point> {
        let mut path = vec![current];
        while came_from.contains_key(&current) {
            current = *came_from.get(&current).unwrap();
            path.push(current);
        }
        path.reverse();
        path
    }
}

impl PathfindingAlgorithm for AStar {
    fn find_path(&mut self, visualizer: &mut MazeVisualization) -> bool {
        // Find entrance and exit coordinates
        let entrance = visualizer.maze.get_entrance().unwrap();
        let exit = visualizer.maze.get_entrance().unwrap();

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
            h: self.heuristic(&start, &goal),
        });
        g_scores.insert(start, 0);

        while let Some(current_node) = open_set.pop() {
            let current = current_node.point;

            if current == goal {
                let path = Self::reconstruct_path(came_from, current);
                for point in path.iter().skip(1) {
                    visualizer
                        .maze
                        .set_cell(point.x, point.y, MazeCell::FinalPath);
                    visualizer.draw(self.name());
                    thread::sleep(Duration::from_millis(self.visualization_delay));
                }
                return true;
            }

            for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let neighbor = Point {
                    x: (current.x as i32 + dx) as usize,
                    y: (current.y as i32 + dy) as usize,
                };

                if !visualizer
                    .maze
                    .is_valid_move(neighbor.x as i32, neighbor.y as i32)
                    || visualizer.maze.get_cell(neighbor.x, neighbor.y) == MazeCell::Wall
                {
                    continue;
                }

                let tentative_g_score = g_scores[&current] + 1;

                if !g_scores.contains_key(&neighbor) || tentative_g_score < g_scores[&neighbor] {
                    g_scores.insert(neighbor, tentative_g_score);
                    came_from.insert(neighbor, current);

                    open_set.push(Node {
                        point: neighbor,
                        g: tentative_g_score,
                        h: self.heuristic(&neighbor, &goal),
                    });
                }
            }
        }

        false
    }

    fn name(&self) -> &str {
        "A*"
    }
}
