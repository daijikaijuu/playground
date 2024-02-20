use ui::Ui;

mod astar;
mod backtracking;
mod maze;
mod pathfinding;
mod ui;
mod visualization;

fn main() {
    let mut ui = Ui::new();
    ui.run();
}
