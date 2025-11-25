use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, poll};

use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    text::Line,
    widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, Wrap},
};

use crate::{Result, app::App};

pub fn tui_loop(terminal: &mut DefaultTerminal, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|frame| draw_logic(frame, app))?;

        if poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                let should_quit = handle_key_event(key, app);
                if should_quit {
                    break;
                }
            }
        }
    }

    ratatui::restore();

    Ok(())
}

fn handle_key_event(key: event::KeyEvent, app: &mut App) -> bool {
    match key.code {
        // Handle Quitting
        KeyCode::Char('q') | KeyCode::Esc | KeyCode::Left | KeyCode::Char('h') => return true,
        // Handle scrolling Lore Up I guess
        KeyCode::Char('j') => {
            app.scroll_down();
        }
        KeyCode::Char('k') => {
            app.scroll_up();
        }
        _ => {}
    }
    false
}

pub fn draw_logic(frame: &mut Frame, app: &mut App) {
    let mut controls_lines = vec![];
    let controls_txts = [
        "Use j/k to scroll the lore. Use e to enter edit mode to define your sql.",
        "Use t to enter talk mode. Use [enter] to test your solution.",
    ];
    for controls_txt in controls_txts {
        controls_lines.push(Line::from(controls_txt));
    }

    use Constraint::{Fill, Length, Min};

    let vertical = Layout::vertical([Length(3), Min(0), Length(4), Length(3)]);
    let [title_area, main_area, controls_area, bottom_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Fill(1); 2]);
    let [lore_area, output_area] = horizontal.areas(main_area);

    let title_block = Block::bordered().title(Line::from("STAGE").centered());
    let instructions_block = Block::bordered().title("INSTRUCTIONS");
    let lore_block = Block::bordered().title("LORE");
    let controls_block = Block::bordered().title(Line::from("").centered());
    let bottom_block = Block::bordered().title(Line::from("CPU").centered());

    // let lore_text = Paragraph::new(app.lore.clone())
    //     .block(lore_block)
    //     .wrap(Wrap { trim: true });

    let lore_text = Paragraph::new(app.lore.clone())
        .block(lore_block)
        .scroll((app.lore_scroll as u16, 0));

    // frame.render_widget(lore_text, lore_area);

    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        lore_area,
        &mut app.lore_scroll_state,
    );

    let title_text = Paragraph::new(app.level.clone())
        .block(title_block)
        .centered()
        .wrap(Wrap { trim: true });

    // let inst = instructions;
    // let markdown = tui_markdown::from_str(&inst);
    // let instructions_text = Paragraph::new(markdown)
    //     .block(instructions_block)
    //     .wrap(Wrap { trim: true });

    // let instructions_text = Paragraph::new(self.instructions)
    //     .block(instructions_block)
    //     .wrap(Wrap { trim: true });

    let instructions_text = Paragraph::new(app.output.clone())
        .block(instructions_block)
        .wrap(Wrap { trim: true });

    let controls_text = Paragraph::new(controls_lines)
        .block(controls_block)
        .centered()
        .wrap(Wrap { trim: true });

    frame.render_widget(title_text, title_area);
    frame.render_widget(instructions_text, output_area);
    frame.render_widget(lore_text, lore_area);
    frame.render_widget(controls_text, controls_area);

    frame.render_widget(bottom_block, bottom_area);
}
