use crossterm::event::{self, KeyCode};

use crate::app::App;
use crate::tui::EventResult;

pub fn handle_key_event_map_view(key: event::KeyEvent, app: &mut App) -> EventResult {
    let (r, c) = app.player;

    let target = match key.code {
        KeyCode::Char('q') | KeyCode::Esc | KeyCode::Char('m') => {
            app.cycle_view();
            return EventResult::Loop;
        }
        KeyCode::Char('h') => (r, c.saturating_sub(1)),
        KeyCode::Char('l') => (r, c + 1),
        KeyCode::Char('k') => (r.saturating_sub(1), c),
        KeyCode::Char('j') => (r + 1, c),
        _ => return EventResult::Loop,
    };

    if app.map[target.0][target.1] != '#' {
        app.player = target;
    }

    EventResult::Loop
}
