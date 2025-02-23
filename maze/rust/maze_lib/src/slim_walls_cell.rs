#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum SlimWallsCellType {
    #[default]
    Path,
    Entrance,
    Exit,
    Visited,
    FinalPath,
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct SlimWallsCell {
    pub walls: u8,
    pub cell: SlimWallsCellType,
}

impl SlimWallsCell {
    const LEFT_WALL: u8 = 0b1000;
    const RIGHT_WALL: u8 = 0b0100;
    const TOP_WALL: u8 = 0b0010;
    const BOTTOM_WALL: u8 = 0b0001;

    pub fn new() -> SlimWallsCell {
        SlimWallsCell {
            walls: 0,
            cell: SlimWallsCellType::Path,
        }
    }

    pub fn has_left_wall(&self) -> bool {
        self.walls & Self::LEFT_WALL != 0
    }

    pub fn set_left_wall(&mut self, has_wall: bool) {
        if has_wall {
            self.walls |= Self::LEFT_WALL;
        } else {
            self.walls &= !Self::LEFT_WALL;
        }
    }

    pub fn has_right_wall(&self) -> bool {
        self.walls & Self::RIGHT_WALL != 0
    }

    pub fn set_right_wall(&mut self, has_wall: bool) {
        if has_wall {
            self.walls |= Self::RIGHT_WALL;
        } else {
            self.walls &= !Self::RIGHT_WALL;
        }
    }

    pub fn has_top_wall(&self) -> bool {
        self.walls & Self::TOP_WALL != 0
    }

    pub fn set_top_wall(&mut self, has_wall: bool) {
        if has_wall {
            self.walls |= Self::TOP_WALL;
        } else {
            self.walls &= !Self::TOP_WALL;
        }
    }

    pub fn has_bottom_wall(&self) -> bool {
        self.walls & Self::BOTTOM_WALL != 0
    }

    pub fn set_bottom_wall(&mut self, has_wall: bool) {
        if has_wall {
            self.walls |= Self::BOTTOM_WALL;
        } else {
            self.walls &= !Self::BOTTOM_WALL;
        }
    }
}
