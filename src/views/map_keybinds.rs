use crossterm::event::{self, KeyCode};

use crate::app::App;
use crate::tui::EventResult;

pub fn handle_key_event_map_view(key: event::KeyEvent, app: &mut App) -> EventResult {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc | KeyCode::Char('m') => {
            app.cycle_view();
            return EventResult::Loop;
        }
        _ => EventResult::Loop,
    }
}
