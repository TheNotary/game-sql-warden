use crossterm::event::{self, KeyCode};

use crate::app::App;
use crate::tui_loop::EventResult;

pub fn handle_key_popup(key: event::KeyEvent, app: &mut App) -> EventResult {
    match key.code {
        KeyCode::Char('y') => {
            app.show_popup = false;

            if let Some(mut task) = app.confirmation_task.take() {
                task(app);
            } else {
                panic!(
                    "A popup response was yes, but there was no confirmation_task... I don't think this can happen, but...."
                )
            }
            EventResult::Loop
        }
        KeyCode::Char('n') | KeyCode::Esc => {
            app.show_popup = false;
            EventResult::Loop
        }
        _ => EventResult::Loop,
    }
}
