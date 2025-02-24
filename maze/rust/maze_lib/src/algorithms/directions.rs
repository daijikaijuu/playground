use super::Point;

pub struct Movements {}

impl Movements {
    pub fn directions() -> [(i32, i32); 4] {
        [(0, 1), (1, 0), (0, -1), (-1, 0)]
    }
    pub fn directions_doubled() -> [(i32, i32); 4] {
        [(0, 2), (2, 0), (0, -2), (-2, 0)]
    }

    pub fn calculate_direction(current: Point, neighbor: Point) -> (i32, i32) {
        (
            current.x as i32 - neighbor.x as i32,
            current.y as i32 - neighbor.y as i32,
        )
    }

    pub fn get_opposite_direction(x: i32, y: i32) -> (i32, i32) {
        match (x, y) {
            (0, 1) => (0, -1),
            (1, 0) => (-1, 0),
            (0, -1) => (0, 1),
            (-1, 0) => (1, 0),
            _ => unreachable!("Invalid direction"),
        }
    }
}
