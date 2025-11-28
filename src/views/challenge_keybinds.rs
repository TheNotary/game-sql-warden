use log::trace;
use std::process::{Command, Stdio};

use crossterm::event::{self, KeyCode};

use crate::tui::EventResult;
use crate::{
    DB_PATH, SOLUTION_PATH,
    app::{App, RightPaneMode},
};

pub fn handle_key_event_challenge_view(key: event::KeyEvent, app: &mut App) -> EventResult {
    match key.code {
        // Quit
        KeyCode::Char('q') | KeyCode::Esc => {
            return EventResult::Quit;
        }
        // Map Screen
        KeyCode::Char('m') => {
            app.cycle_view_to_map();
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
            trace!("Enter key pressed");

            // if the stage is already cleared, don't touch the db again
            if app.game_state.cleared_levels.contains(&app.stage.id) {
                app.right_pane_mode = RightPaneMode::Output;
                app.stage.output = String::from("🏆 You have cleared this stage!");
                return EventResult::Loop;
            }

            match app.execute_solution() {
                Err(err) => {
                    app.stage.output = format!("Your SQL did not apply well: {}", err);
                    app.right_pane_mode = RightPaneMode::Output;
                    return EventResult::Loop;
                }
                _ => {}
            }
            app.assess_db()
                .expect("Error: Something went wrong assessing your solution and the database =/");
            app.right_pane_mode = RightPaneMode::Output;
        }
        // Enter SQLite Console
        KeyCode::Char('/') | KeyCode::Char('.') | KeyCode::Char(',') => {
            ratatui::restore();
            let _ = run_sqlite(&app.stage.base_dir);
            app.stage.output = String::new();
            return EventResult::ReloadTerminal;
        }
        // Edit solution.sql
        KeyCode::Char('e') => {
            ratatui::restore();
            // let _ = run_nano_lol();
            let _ = run_vi(&app.stage.base_dir);
            app.stage.output = String::new();
            return EventResult::ReloadTerminal;
        }
        _ => {}
    }
    EventResult::Loop
}

fn run_sqlite(base_dir: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("CTRL + D to exit\n");
    let mut child = Command::new("sqlite3")
        .arg(format!("{base_dir}/{DB_PATH}"))
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

fn run_vi(base_dir: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new("vi")
        .arg(format!("{base_dir}/{SOLUTION_PATH}"))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    // Wait until user exits with .quit or CTRL+D
    child.wait()?;

    Ok(())
}
