use crossterm::event::{self, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Block, Paragraph, Wrap};

use crate::app::App;
use crate::views::challenge_view::EventResult;

pub fn draw_map_view(frame: &mut Frame<'_>, app: &mut App) {
    use Constraint::{Fill, Length, Min};

    //      Title
    // ----- Map ------
    // ----------------
    // ----------------
    //      Legend
    let vertical = Layout::vertical([Length(3), Min(0), Length(4)]);

    let [title_area, map_area, legend_area] = vertical.areas(frame.area());

    let title_block = Block::bordered();
    let title_text = Paragraph::new("MAP".to_string())
        .block(title_block)
        .centered()
        .bg(Color::Gray)
        .wrap(Wrap { trim: true });

    frame.render_widget(title_text, title_area);
}

pub fn handle_key_event_map_view(key: event::KeyEvent, app: &mut App) -> EventResult {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc | KeyCode::Char('m') => {
            app.cycle_view();
            return EventResult::Loop;
        }
        _ => EventResult::Loop,
    }
}
