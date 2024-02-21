use iced::{
    mouse,
    widget::{
        canvas::{self, Geometry},
        Canvas,
    },
    Element, Length, Rectangle, Renderer, Theme,
};

use crate::maze::Maze;

#[derive(Default)]
pub struct MazeGrid {
    maze: Maze,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {}

impl MazeGrid {
    pub fn new() -> Self {
        MazeGrid {
            maze: Maze::new(41, 41),
        }
    }

    pub fn view(&self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl canvas::Program<Message> for MazeGrid {
    type State = Interaction;

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        todo!()
    }
}

pub enum Interaction {
    None,
}

impl Default for Interaction {
    fn default() -> Self {
        Self::None
    }
}
