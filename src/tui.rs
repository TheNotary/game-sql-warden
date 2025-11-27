use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::time::Duration;

use crossterm::event::{self, Event, poll};
use notify::{EventKind, FsEventWatcher, RecursiveMode, Watcher};
use ratatui::Frame;

use crate::app::View;
use crate::views::no_stage_keybinds::handle_key_no_stage;
use crate::views::no_stage_view::draw_no_stage_view;
use crate::{
    SOLUTION_PATH,
    api::Result,
    app::App,
    views::{
        challenge_keybinds::handle_key_event_challenge_view, challenge_view::draw_challenge_view,
        map_keybinds::handle_key_event_map_view, map_view::draw_map_view,
    },
};

pub enum EventResult {
    Quit,
    Loop,
    ReloadTerminal,
}

pub fn tui_loop(app: &mut App) -> Result<()> {
    let mut terminal = ratatui::init();
    let (_watcher, rx) = setup_file_watcher(&app.stage.base_dir).expect("COMPUTER!");

    loop {
        terminal.draw(|frame| draw_logic(frame, app))?;

        // every 50ms, check for inputs from user
        if poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match handle_key_event(key, app) {
                    EventResult::Quit => break,
                    EventResult::Loop => continue,
                    EventResult::ReloadTerminal => terminal = ratatui::init(),
                }
            }
        }

        // FIXME: Move to function? At least clean up?
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
    match app.current_view {
        View::ChallengeScreen => handle_key_event_challenge_view(key, app),
        View::MapScreen => handle_key_event_map_view(key, app),
        View::NoStage => handle_key_no_stage(key, app),
    }
}

fn draw_logic(frame: &mut Frame, app: &mut App) {
    match app.current_view {
        View::ChallengeScreen => draw_challenge_view(frame, app),
        View::MapScreen => draw_map_view(frame, app),
        View::NoStage => draw_no_stage_view(frame, app),
    }
}
