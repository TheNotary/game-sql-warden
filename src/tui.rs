use crossterm::event::{self, Event};

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    text::Line,
    widgets::{Block, Paragraph},
};

use crate::{Result, app::App};

pub fn tui_loop(app: &App) -> Result<()> {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(|meh| draw(meh, app))?;
        if matches!(event::read()?, Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
    Ok(())
}

fn draw(frame: &mut Frame, app: &App) {
    use Constraint::{Fill, Length, Min};

    let vertical = Layout::vertical([Length(3), Min(0), Length(3)]);
    let [title_area, main_area, status_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Fill(1); 2]);
    let [lore_area, output_area] = horizontal.areas(main_area);

    let title_block = Block::bordered().title(Line::from("STAGE").centered());
    let output_block = Block::bordered().title("CPU");
    let input_block = Block::bordered().title(Line::from("INPUT").centered());
    let lore_block = Block::bordered().title("LORE");

    let lore_text = Paragraph::new(app.lore.clone()).block(lore_block);
    let title_text = Paragraph::new(app.level.clone())
        .block(title_block)
        .centered();
    let output_text = Paragraph::new(app.output.clone()).block(output_block);

    frame.render_widget(title_text, title_area);
    frame.render_widget(output_text, output_area);
    frame.render_widget(lore_text, lore_area);

    frame.render_widget(input_block, status_area);

    // frame.render_widget(lore_block, lore_area);
    // frame.render_widget(title_block, title_area);
    // frame.render_widget(output_block, output_area);
}
