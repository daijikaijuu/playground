#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 1, y: 1 }
    }
}
