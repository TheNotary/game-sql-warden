use crossterm::event::{self, KeyCode};

use crate::app::App;
use crate::tui_loop::EventResult;

pub fn handle_key_no_stage(key: event::KeyEvent, app: &mut App) -> EventResult {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc | KeyCode::Char('m') => {
            app.cycle_view_to_map();

            EventResult::Loop
        }
        KeyCode::Char('p') => EventResult::Quit,
        _ => EventResult::Loop,
    }
}
