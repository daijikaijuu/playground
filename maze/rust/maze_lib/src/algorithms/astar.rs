use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    sync::mpsc::Sender,
};

use crate::maze::Maze;

use super::{pathfinding::PathfindingAlgorithm, Algorithm, Movements, PathfindingResult, Point};

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

#[derive(Default)]
pub struct AStar {}

impl AStar {
    pub fn new() -> Self {
        AStar {}
    }

    fn heuristic(&self, current: &Point, goal: &Point) -> u32 {
        // Simple Manhattan distance as the heuristic
        ((current.x as i32 - goal.x as i32).abs() + (current.y as i32 - goal.y as i32).abs()) as u32
    }

    fn reconstruct_path(
        came_from: &HashMap<Point, Point>,
        mut current: Point,
        // running_flag: Arc<Mutex<bool>>,
    ) -> Vec<Point> {
        let mut path = vec![current];
        while came_from.contains_key(&current) {
            //&& *running_flag.lock().unwrap() {
            current = *came_from.get(&current).unwrap();
            path.push(current);
        }
        path.reverse();
        path
    }
}

impl PathfindingAlgorithm for AStar {
    fn find_path(&mut self, maze: &mut Maze, sender: &Sender<PathfindingResult>) {
        // Find entrance and exit coordinates
        let entrance = maze.get_entrance().unwrap();
        let exit = maze.get_exit().unwrap();

        let start = entrance;
        let goal = exit;

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
            maze.mark_cell_as_visited(current);
            sender
                .send(PathfindingResult {
                    maze: maze.clone(),
                    stats: self.get_stats(),
                })
                .unwrap();

            if current == goal {
                let path = AStar::reconstruct_path(&came_from, current);
                for point in path.iter().skip(1) {
                    maze.mark_cell_as_final_path(*point);

                    sender
                        .send(PathfindingResult {
                            maze: maze.clone(),
                            stats: None,
                        })
                        .expect("Failed to send maze to the main thread");
                }
                break;
            }

            for (dx, dy) in &Movements::directions() {
                let neighbor = Point {
                    x: (current.x as i32 + dx) as usize,
                    y: (current.y as i32 + dy) as usize,
                };

                if !maze.is_valid_coord(neighbor.x as i32, neighbor.y as i32)
                    || maze.is_not_passable(current, neighbor)
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
    }

    fn name(&self) -> Algorithm {
        Algorithm::AStar
    }

    fn get_stats(&self) -> Option<super::PathfindingStats> {
        None
    }
}
