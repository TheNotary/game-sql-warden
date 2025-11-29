use crossterm::event::{self, KeyCode};

use crate::app::App;
use crate::tui_loop::EventResult;

pub fn handle_key_event_map_view(key: event::KeyEvent, app: &mut App) -> EventResult {
    let (r, c) = app.game_state.player;

    let target = match key.code {
        KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter | KeyCode::Char('m') => {
            app.update_current_stage();
            return EventResult::Loop;
        }
        KeyCode::Left | KeyCode::Char('h') => (r, c.saturating_sub(1)),
        KeyCode::Right | KeyCode::Char('l') => (r, c + 1),
        KeyCode::Up | KeyCode::Char('k') => (r.saturating_sub(1), c),
        KeyCode::Down | KeyCode::Char('j') => (r + 1, c),
        _ => return EventResult::Loop,
    };

    app.game_state.set_player(target);

    EventResult::Loop
}
