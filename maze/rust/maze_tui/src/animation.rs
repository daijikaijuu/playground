#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum AnimationState {
    #[default]
    NotRunning,
    Running,
    Paused,
}
