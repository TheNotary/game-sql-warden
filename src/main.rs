use std::fs::read_to_string;

use crate::{
    api::{ChallengeError, Result, assess_db_condition, delete_db_file, handle_db_condition},
    app::App,
    tui::tui_loop,
};

mod api;
mod app;
mod evaluation;
mod presenter;
mod tui;

pub static DB_PATH: &str = "database.db";
pub static LORE_PATH: &str = "01_lore.md";
pub static INSTRUCTIONS_PATH: &str = "02_instructions.md";
pub static MIGRATION_PATH: &str = "03_migration.sql";
pub static TEST_SQL_PATH: &str = "04_test.sql";

fn main() -> Result<()> {
    let mut terminal = ratatui::init();

    match run_program() {
        Ok(mut app) => tui_loop(&mut terminal, &mut app),
        Err(ChallengeError::MigrationFailed) => {
            eprintln!("❌ sqlite3 failed to apply migration");
            delete_db_file(DB_PATH)
        }
        Err(ChallengeError::MigrationFileMissing(_)) => {
            eprintln!("❌ {MIGRATION_PATH} missing — cannot build {DB_PATH}");
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn run_program() -> Result<App> {
    let level = "lvl 1 - Strongest Cubical".to_string();
    let lore = read_to_string(LORE_PATH).expect(&format!("Unable to read {LORE_PATH}."));
    let instructions =
        read_to_string(INSTRUCTIONS_PATH).expect(&format!("Unable to read {INSTRUCTIONS_PATH}."));
    let output = handle_db_condition(assess_db_condition(DB_PATH)?)?;

    let app = App::new(level, lore, instructions, output);

    Ok(app)
}
