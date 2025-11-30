use crossterm::event::{self, KeyCode};
use ratatui::widgets::ListState;

use crate::app::App;
use crate::tui_loop::EventResult;

pub fn handle_key_title_screen(
    key: event::KeyEvent,
    app: &mut App,
    title_state: &mut ListState,
) -> EventResult {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc | KeyCode::Char('m') => EventResult::Quit,
        KeyCode::Up | KeyCode::Char('k') => {
            title_state.select_previous();
            EventResult::Loop
        }
        KeyCode::Down | KeyCode::Char('j') => {
            title_state.select_next();
            EventResult::Loop
        }
        KeyCode::Char('l') | KeyCode::Enter => {
            // Check choice from title_state
            // And implement deleting all the databases I guess
            match title_state.selected() {
                Some(0) => app.cycle_view_to_map(),
                Some(1) => app.cycle_view_to_map(),
                Some(2) => todo!(),
                _ => {}
            }

            EventResult::Loop
        }
        _ => EventResult::Loop,
    }
}
