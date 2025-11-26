use std::process::{Command, Stdio};
use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, poll};
use notify::{EventKind, FsEventWatcher, RecursiveMode, Watcher};
use ratatui::style::{Color, Stylize};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    text::Line,
    widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, Wrap},
};

use crate::SOLUTION_PATH;
use crate::app::RightPaneMode;
use crate::{
    api::Result,
    app::{App, LeftPaneMode},
};

enum EventResult {
    Quit,
    Loop,
    ReloadTerminal,
}

pub fn tui_loop(app: &mut App) -> Result<()> {
    let mut terminal = ratatui::init();
    let (_watcher, rx) = setup_file_watcher(&app.base_dir).expect("COMPUTER!");

    loop {
        terminal.draw(|frame| draw_logic(frame, app))?;

        // every 150ms, check for inputs from user
        if poll(Duration::from_millis(150))? {
            if let Event::Key(key) = event::read()? {
                match handle_key_event(key, app) {
                    EventResult::Quit => break,
                    EventResult::Loop => continue,
                    EventResult::ReloadTerminal => terminal = ratatui::init(),
                }
            }
        }

        // FIXME: Move to function?
        match rx.recv_timeout(Duration::from_millis(10)) {
            Ok(Ok(event)) => {
                if let EventKind::Modify(_) = event.kind {
                    app.reload_solution_file();
                }
            }
            Ok(Err(e)) => println!("Watcher error: {:?}", e),
            Err(RecvTimeoutError::Timeout) => {}
            Err(e) => println!("Channel error: {:?}", e),
        }
    }

    ratatui::restore();

    Ok(())
}

fn setup_file_watcher(
    base_dir: &str,
) -> notify::Result<(FsEventWatcher, Receiver<notify::Result<notify::Event>>)> {
    let solution_path = format!("{base_dir}/{SOLUTION_PATH}");
    let (tx, rx) = channel();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(solution_path.as_ref(), RecursiveMode::NonRecursive)?;
    Ok((watcher, rx))
}

fn handle_key_event(key: event::KeyEvent, app: &mut App) -> EventResult {
    match key.code {
        // Quit
        KeyCode::Char('q') | KeyCode::Esc => {
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
        // Cycle Output/ solution.sql
        KeyCode::Backspace | KeyCode::Delete => {
            app.cycle_right_pane();
        }
        // Test solution.sql
        KeyCode::Enter => {
            app.assess_db()
                .expect("Error: Something went wrong assessing your solution and the database =/");
            app.right_pane_mode = RightPaneMode::Output;
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
            ratatui::restore();
            // let _ = run_nano_lol();
            let _ = run_vi();
            app.output = String::new();
            return EventResult::ReloadTerminal;
        }
        _ => {}
    }
    EventResult::Loop
}

pub fn draw_logic(frame: &mut Frame, app: &mut App) {
    let mut controls_lines = vec![];
    let controls_txts = [
        // "‣ [tab] cycle lore/instructions.   ‣ [del] cycle output/solution.",
        "‣ [j/k] scroll the lore.           ‣ [.,/] enter sqlite shell.",
        "‣ [e] edit solution.sql            ‣ [enter] test your solution.",
    ];
    for controls_txt in controls_txts {
        controls_lines.push(Line::from(controls_txt));
    }

    use Constraint::{Fill, Length, Min};

    let vertical = Layout::vertical([Length(3), Min(0), Length(4)]);
    let [title_area, main_area, controls_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Fill(1); 2]);
    let [lore_area, output_area] = horizontal.areas(main_area);

    let (left_pane_title, left_pane_content) = match app.left_pane_mode {
        LeftPaneMode::Lore => ("LORE", app.lore.clone()),
        LeftPaneMode::Instructions => ("INSTRUCTIONS", app.instructions.clone()),
    };

    let (right_pane_title, right_pane_content) = match app.right_pane_mode {
        RightPaneMode::Output => ("OUTPUT", app.output.clone()),
        RightPaneMode::Solution => ("SOLUTION", app.solution.clone()),
    };

    let title_block = Block::bordered().title(Line::from("STAGE").centered());
    let left_pane_block = Block::bordered()
        .title(left_pane_title)
        .title_bottom(Line::from(" [tab] cycle lore/instructions ").centered());
    let right_page_block = Block::bordered()
        .title(right_pane_title)
        .title_bottom(Line::from(" [del] cycle output/solution. ").centered());
    let controls_block = Block::bordered().title(Line::from("CONTROLS").centered());

    let left_pane_text = Paragraph::new(left_pane_content)
        .block(left_pane_block)
        .wrap(Wrap { trim: true })
        .bg(Color::Gray)
        .scroll((app.left_pane_scroll as u16, 0));

    let right_pane_text = Paragraph::new(right_pane_content)
        .block(right_page_block)
        .bg(Color::Gray)
        .wrap(Wrap { trim: true });

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
        .bg(Color::Gray)
        .wrap(Wrap { trim: true });

    let controls_text = Paragraph::new(controls_lines)
        .block(controls_block)
        .bg(Color::Gray)
        .centered();

    frame.render_widget(title_text, title_area);
    frame.render_widget(right_pane_text, output_area);
    frame.render_widget(left_pane_text, lore_area);
    frame.render_widget(controls_text, controls_area);
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

// fn run_nano_lol() -> std::result::Result<(), Box<dyn std::error::Error>> {
//     let mut child = Command::new("nano")
//         .arg("solution.sql")
//         .stdin(Stdio::inherit())
//         .stdout(Stdio::inherit())
//         .stderr(Stdio::inherit())
//         .spawn()?;

//     // Wait until user exits with .quit or CTRL+D
//     child.wait()?;

//     Ok(())
// }

fn run_vi() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new("vi")
        .arg("solution.sql")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    // Wait until user exits with .quit or CTRL+D
    child.wait()?;

    Ok(())
}
