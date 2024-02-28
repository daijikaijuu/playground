use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, AppResult};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.exit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.exit();
            } else {
                app.reset_maze();
            }
        }

        // KeyCode::Up => self.selected_algorithm = previous_cycle(&self.selected_algorithm),
        // KeyCode::Down => self.selected_algorithm = next_cycle(&self.selected_algorithm),
        // KeyCode::Enter => self.find_path(),
        _ => {}
    }
    Ok(())
}
