#[derive(Debug)]
pub enum ScreenState {
    Menu,
    Pathfinding,
}

#[derive(Debug)]
pub struct MainWindow {
    pub screen_state: ScreenState,
}

impl MainWindow {
    pub fn new() -> Self {
        MainWindow {
            screen_state: ScreenState::Menu,
        }
    }

    fn swap_screen_state(screen_state: &ScreenState) -> ScreenState {
        match screen_state {
            ScreenState::Menu => ScreenState::Pathfinding,
            ScreenState::Pathfinding => ScreenState::Menu,
        }
    }
}
