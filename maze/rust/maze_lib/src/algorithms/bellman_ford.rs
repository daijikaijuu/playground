use std::{collections::HashMap, sync::mpsc::Sender};

use crate::{algorithms::MOVEMENTS, Maze, MazeCell};

use super::{PathfindingAlgorithm, PathfindingResult, Point};

#[derive(Default)]
pub struct BellmanFord;

impl BellmanFord {}

impl PathfindingAlgorithm for BellmanFord {
    fn find_path(
        &mut self,
        maze: &mut crate::Maze,
        sender: &std::sync::mpsc::Sender<PathfindingResult>,
    ) {
        let entrance = maze.get_entrance().expect("Entrance not found");
        let exit = maze.get_exit().expect("Exit not found");

        let start = Point {
            x: entrance.0,
            y: entrance.1,
        };
        let goal = Point {
            x: exit.0,
            y: exit.1,
        };

        let mut distance: HashMap<Point, i32> = HashMap::new();
        let mut predecessor: HashMap<Point, Point> = HashMap::new();

        for y in 0..maze.height {
            for x in 0..maze.width {
                let point = Point { x, y };
                distance.insert(point, i32::MAX);
            }
        }

        distance.insert(start, 0);

        for _ in 0..(maze.width * maze.height) - 1 {
            for y in 0..maze.height {
                for x in 0..maze.width {
                    let current = Point { x, y };

                    for (dx, dy) in &MOVEMENTS {
                        let neighbor = Point {
                            x: (x as i32 + dx) as usize,
                            y: (y as i32 + dy) as usize,
                        };

                        if !maze.is_valid_move(neighbor.x as i32, neighbor.y as i32)
                            || maze.get_cell(neighbor.x, neighbor.y) == MazeCell::Wall
                        {
                            continue;
                        }

                        let weight = 1;

                        let tentative_distance = distance[&current] + weight;

                        if tentative_distance < distance[&neighbor] {
                            distance.insert(neighbor, tentative_distance);
                            predecessor.insert(neighbor, current);
                        }
                    }
                }
            }
        }

        // Check for negative cycles
        for y in 0..maze.height {
            for x in 0..maze.width {
                let current = Point {x, y};

                for (dx, dy) in &MOVEMENTS {
                    let neighbor = Point {
                        x: (current.x as i32 + dx) as usize,
                        y: (current.y as i32 + dy) as usize,
                    };

                    if !maze.is_valid_move(neighbor.x as i32, neighbor.y as i32)
                        || maze.get_cell(neighbor.x, neighbor.y) == MazeCell::Wall
                    {
                        continue;
                    }

                    let weight = 1;

                    let tentative_distance = distance[&current] + weight;

                    if tentative_distance < distance[&neighbor] {
                        return;
                    }
                }
            }
        }

        // Reconstruct the path
        let mut current = goal;
        let mut path = Vec::new();

        while let Some(&pred) = predecessor.get(&current) {
            path.push(current);
            current = pred;
        }

        path.push(start);
        path.reverse();

        for point in path.iter().skip(1) {
            maze.set_cell(point.x, point.y, MazeCell::FinalPath);

            sender.send(PathfindingResult {
                stats: None,
                maze: maze.clone()
            }).expect("Failed to send pathfinding result");
        }
    }

    fn get_stats(&self) -> Option<super::PathfindingStats> {
        todo!()
    }

    fn name(&self) -> super::Algorithm {
        super::Algorithm::BellmanFord
    }
}
