use std::process::{Command, Stdio};
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, poll};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    text::Line,
    widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, Wrap},
};

use crate::{
    DB_PATH,
    api::{Result, assess_db_condition, handle_db_condition},
    app::{App, LeftPaneMode},
};

enum EventResult {
    Quit,
    Loop,
    ReloadTerminal,
}

pub fn tui_loop(app: &mut App) -> Result<()> {
    let mut terminal = ratatui::init();

    loop {
        terminal.draw(|frame| draw_logic(frame, app))?;

        if poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                match handle_key_event(key, app) {
                    EventResult::Quit => break,
                    EventResult::Loop => continue,
                    EventResult::ReloadTerminal => terminal = ratatui::init(),
                }
            }
        }
    }

    ratatui::restore();

    Ok(())
}

fn handle_key_event(key: event::KeyEvent, app: &mut App) -> EventResult {
    match key.code {
        // Quit
        KeyCode::Char('q') | KeyCode::Esc | KeyCode::Left | KeyCode::Char('h') => {
            return EventResult::Quit;
        }
        // Scroll Down
        KeyCode::Char('j') => {
            app.scroll_down();
        }
        // Scroll Up
        KeyCode::Char('k') => {
            app.scroll_up();
        }
        // Cycle Lore/ Instructions
        KeyCode::Tab => {
            app.cycle_left_pane();
        }
        // Test solution.sql
        KeyCode::Enter => {
            if let Ok(output) = assess_db_condition(DB_PATH).and_then(handle_db_condition) {
                app.output = output;
            } else {
                app.output =
                    "Error: Something went wrong assessing your solution and the database =/"
                        .to_string();
            }
        }
        // Enter SQLite Console
        KeyCode::Char('/') | KeyCode::Char('.') | KeyCode::Char(',') => {
            ratatui::restore();
            let _ = run_sqlite();
            app.output = String::new();
            return EventResult::ReloadTerminal;
        }
        // Edit solution.sql
        KeyCode::Char('e') => {
            todo!();
        }
        _ => {}
    }
    EventResult::Loop
}

pub fn draw_logic(frame: &mut Frame, app: &mut App) {
    let mut controls_lines = vec![];
    let controls_txts = [
        "‣ [j/k] scroll the lore.          ‣ [.,/] edit solution.sql",
        "‣ [tab] cycle lore/instructions.  ‣ [enter] test your solution.",
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
    let right_page_block = Block::bordered().title("OUTPUT");
    let left_pane_block = Block::bordered().title("LORE");
    let controls_block = Block::bordered().title(Line::from("").centered());
    let bottom_block = Block::bordered().title(Line::from("CPU").centered());

    let left_pane_content = match app.left_pane_mode {
        LeftPaneMode::Lore => app.lore.clone(),
        LeftPaneMode::Instructions => app.instructions.clone(),
    };

    let left_pane_text = Paragraph::new(left_pane_content)
        .block(left_pane_block)
        .wrap(Wrap { trim: true })
        .scroll((app.left_pane_scroll as u16, 0));

    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        lore_area,
        &mut app.left_pane_scroll_state,
    );

    let title_text = Paragraph::new(app.level.clone())
        .block(title_block)
        .centered()
        .wrap(Wrap { trim: true });

    let right_pane_text = Paragraph::new(app.output.clone())
        .block(right_page_block)
        .wrap(Wrap { trim: true });

    let controls_text = Paragraph::new(controls_lines)
        .block(controls_block)
        .centered()
        .wrap(Wrap { trim: true });

    frame.render_widget(title_text, title_area);
    frame.render_widget(right_pane_text, output_area);
    frame.render_widget(left_pane_text, lore_area);
    frame.render_widget(controls_text, controls_area);

    frame.render_widget(bottom_block, bottom_area);
}

pub fn run_sqlite() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("CTRL + D to exit\n");
    let mut child = Command::new("sqlite3")
        .arg("database.db")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    // Wait until user exits with .quit or CTRL+D
    child.wait()?;

    Ok(())
}
