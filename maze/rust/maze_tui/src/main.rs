use std::io;

use app::App;

mod app;
mod tui;
mod maze_grid;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::new().run(&mut terminal);
    tui::restore()?;
    app_result
}

