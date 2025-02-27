#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum CellType {
    Wall,
    #[default]
    Path,
    Entrance,
    Exit,
    Visited,
    FinalPath,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct MazeCell {
    cell_type: CellType,
    pub walls: u8,
}

impl Default for MazeCell {
    fn default() -> Self {
        MazeCell {
            walls: 0,
            walls: 0b1111,
            cell_type: CellType::default(),
        }
    }
}

impl MazeCell {
    pub fn new(cell_type: CellType) -> Self {
        MazeCell {
            cell_type,
            walls: 0,
            walls: 0b1111,
        }
    }

    pub fn mark_cell_as(&mut self, cell_type: CellType) {
        self.cell_type = cell_type;
    }

    pub fn get_type(&self) -> CellType {
        self.cell_type
    }

    pub fn is_entrance(&self) -> bool {
        self.cell_type == CellType::Entrance
    }

    pub fn is_exit(&self) -> bool {
        self.cell_type == CellType::Exit
    }
}

pub trait SlimWallsCellType {
    const LEFT_WALL: u8 = 0b1000;
    const RIGHT_WALL: u8 = 0b0100;
    const TOP_WALL: u8 = 0b0010;
    const BOTTOM_WALL: u8 = 0b0001;

    fn has_left_wall(&self) -> bool;
    fn set_left_wall(&mut self, has_wall: bool);
    fn has_right_wall(&self) -> bool;
    fn set_right_wall(&mut self, has_wall: bool);
    fn has_top_wall(&self) -> bool;
    fn set_top_wall(&mut self, has_wall: bool);
    fn has_bottom_wall(&self) -> bool;
    fn set_bottom_wall(&mut self, has_wall: bool);

    fn set_wall_by_direction(&mut self, direction: (i32, i32), has_wall: bool);
    fn has_wall_in_direction(&self, direction: (i32, i32)) -> bool;
}

impl SlimWallsCellType for MazeCell {
    fn has_left_wall(&self) -> bool {
        self.walls & Self::LEFT_WALL != 0
    }

    fn set_left_wall(&mut self, has_wall: bool) {
        if has_wall {
            self.walls |= Self::LEFT_WALL;
        } else {
            self.walls &= !Self::LEFT_WALL;
        }
    }

    fn has_right_wall(&self) -> bool {
        self.walls & Self::RIGHT_WALL != 0
    }

    fn set_right_wall(&mut self, has_wall: bool) {
        if has_wall {
            self.walls |= Self::RIGHT_WALL;
        } else {
            self.walls &= !Self::RIGHT_WALL;
        }
    }

    fn has_top_wall(&self) -> bool {
        self.walls & Self::TOP_WALL != 0
    }

    fn set_top_wall(&mut self, has_wall: bool) {
        if has_wall {
            self.walls |= Self::TOP_WALL;
        } else {
            self.walls &= !Self::TOP_WALL;
        }
    }

    fn has_bottom_wall(&self) -> bool {
        self.walls & Self::BOTTOM_WALL != 0
    }

    fn set_bottom_wall(&mut self, has_wall: bool) {
        if has_wall {
            self.walls |= Self::BOTTOM_WALL;
        } else {
            self.walls &= !Self::BOTTOM_WALL;
        }
    }

    fn set_wall_by_direction(&mut self, direction: (i32, i32), has_wall: bool) {
        match direction {
            (1, 0) => self.set_right_wall(has_wall),
            (0, 1) => self.set_bottom_wall(has_wall),
            (-1, 0) => self.set_left_wall(has_wall),
            (0, -1) => self.set_top_wall(has_wall),
            _ => unreachable!(),
        };
    }

    fn has_wall_in_direction(&self, direction: (i32, i32)) -> bool {
        match direction {
            (1, 0) => self.has_right_wall(),
            (0, 1) => self.has_bottom_wall(),
            (-1, 0) => self.has_left_wall(),
            (0, -1) => self.has_right_wall(),
            _ => unreachable!(),
        }
    }
}
