use crossterm::event::{self, Event};

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::Style,
    text::{Line, Text},
    widgets::{Block, Paragraph},
};

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

fn draw(frame: &mut Frame, lore_str: &str) {
    let level_name = "lvl 1 - Strongest Cubical";

    use Constraint::{Fill, Length, Min};

    let vertical = Layout::vertical([Length(3), Min(0), Length(3)]);
    let [title_area, main_area, status_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Fill(1); 2]);
    let [lore_area, output_area] = horizontal.areas(main_area);

    let title_block = Block::bordered().title(Line::from("Title Bar").centered());
    let output_block = Block::bordered().title("CPU");
    let status_bar_block = Block::bordered().title("Status Bar");
    let lore_block = Block::bordered().title("Lore");

    let title_text = Paragraph::new(level_name).block(title_block).centered();
    let output_text = Paragraph::new(lore_str).block(output_block);

    frame.render_widget(title_text, title_area);
    frame.render_widget(output_text, output_area);

    frame.render_widget(status_bar_block, status_area);
    frame.render_widget(lore_block, lore_area);

    // frame.render_widget(title_block, title_area);
    // frame.render_widget(output_block, output_area);
}
