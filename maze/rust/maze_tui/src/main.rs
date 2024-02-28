use std::io;

use maze_tui::{
    app::{App, AppResult},
    event::{Event, EventHandler},
    handler::handle_key_events,
    tui::Tui,
};
use ratatui::{backend::CrosstermBackend, Terminal};

#[tokio::main]
async fn main() -> AppResult<()> {
    let mut app = App::new();
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(200);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    while app.running {
        tui.draw(&mut app)?;

        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
        }
    }

    tui.exit()?;

    Ok(())
}
