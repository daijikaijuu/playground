use raylib::prelude::*;

// Structure to represent a bar in the visualization
#[derive(Clone)]
pub struct SortingBar {
    pub value: i32,
    pub color: Color,
}

impl SortingBar {
    pub fn new(value: i32) -> Self {
        let color = Color::new(
            (value % 255) as u8,
            ((value * 2) % 255) as u8,
            ((value * 2) % 255) as u8,
            255,
        );
        Self { value, color }
    }
}
