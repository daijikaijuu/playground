pub trait PathfindingAlgorithm {
    fn find_path(&mut self);
    fn name(&self) -> &str;
}
