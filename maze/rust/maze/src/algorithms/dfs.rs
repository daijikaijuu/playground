use std::{collections::HashSet, sync::mpsc::Sender};

use crate::maze::{Maze, MazeCell};

use super::{PathfindingAlgorithm, Point, MOVEMENTS};

pub struct DFS;

impl DFS {
    pub fn new() -> Self {
        DFS
    }

    fn depth_first_search(
        &self,
        current: Point,
        goal: Point,
        maze: &mut Maze,
        sender: &Sender<Maze>,
        visited: &mut HashSet<Point>,
    ) -> bool {
        visited.insert(current);
        maze.set_cell(current.x, current.y, MazeCell::FinalPath);

        if current == goal {
            return true;
        }

        for (dx, dy) in MOVEMENTS {
            let neighbor = Point {
                x: (current.x as i32 + dx) as usize,
                y: (current.y as i32 + dy) as usize,
            };

            if maze.is_valid_move(neighbor.x as i32, neighbor.y as i32)
                && !visited.contains(&Point {
                    x: neighbor.x as usize,
                    y: neighbor.y as usize,
                })
                && maze.get_cell(neighbor.x, neighbor.y) != MazeCell::Wall
            {
                sender.send(maze.clone()).unwrap();

                // Mark the final path
                maze.set_cell(neighbor.x, neighbor.y, MazeCell::FinalPath);

                if self.depth_first_search(
                    Point {
                        x: neighbor.x,
                        y: neighbor.y,
                    },
                    goal,
                    maze,
                    sender,
                    visited,
                ) {
                    return true;
                } else {
                    visited.remove(&neighbor);
                    maze.set_cell(neighbor.x, neighbor.y, MazeCell::Visited);
                }
            }
        }

        false
    }
}

impl PathfindingAlgorithm for DFS {
    fn find_path(&mut self, maze: &mut Maze, sender: &Sender<Maze>) {
        let entrance = maze.get_entrance().expect("Cannot find entrance");
        let exit = maze.get_exit().expect("Cannot find exit");

        let mut visited = HashSet::new();
        self.depth_first_search(
            Point {
                x: entrance.0,
                y: entrance.1,
            },
            Point {
                x: exit.0,
                y: exit.1,
            },
            maze,
            sender,
            &mut visited,
        );
    }

    fn name(&self) -> super::Algorithm {
        todo!()
    }
}
