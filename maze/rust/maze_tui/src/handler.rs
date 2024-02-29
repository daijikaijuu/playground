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

        KeyCode::Up | KeyCode::Char('k') => app.select_previous_algorithm(),
        KeyCode::Down | KeyCode::Char('j') => app.select_next_algorithm(),
        KeyCode::Enter => app.find_path(),
        KeyCode::Char(' ') => app.pause_unpause_animation(),
        _ => {}
    }
    Ok(())
}
