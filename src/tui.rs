use crossterm::event::{self, Event};
use ratatui::{Frame, text::Text};

use crate::Result;

pub fn tui_loop(resp: &str) -> Result<()> {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(|meh| draw(meh, &resp))?;
        if matches!(event::read()?, Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
    Ok(())
}

fn draw(frame: &mut Frame, msg: &str) {
    let text = Text::raw(msg);
    frame.render_widget(text, frame.area());
}
