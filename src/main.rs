use crate::{
    api::{ChallengeError, Result, delete_db_file},
    app::App,
    tui::tui_loop,
};

mod api;
mod app;
mod evaluation;
mod presenter;
mod tui;

pub static DB_PATH: &str = "database.db";
pub static NAME_PATH: &str = "00_name.txt";
pub static LORE_PATH: &str = "01_lore.md";
pub static INSTRUCTIONS_PATH: &str = "02_instructions.md";
pub static MIGRATION_PATH: &str = "03_migration.sql";
pub static TEST_SQL_PATH: &str = "04_test.sql";
pub static SOLUTION_PATH: &str = "solution.sql";

fn main() -> Result<()> {
    let base_dir = "challenges/01_strongest_cubical";
    match run_program(base_dir) {
        Ok(mut app) => tui_loop(&mut app),
        Err(ChallengeError::MigrationFailed) => {
            eprintln!("❌ sqlite3 failed to apply migration");
            delete_db_file(&format!("{base_dir}/{DB_PATH}"))
        }
        Err(ChallengeError::MigrationFileMissing(migration_path)) => {
            eprintln!("❌ {migration_path} missing — cannot build {base_dir}/{DB_PATH}");
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn run_program(challenge_dir: &str) -> Result<App> {
    let mut app = App::from_dir(challenge_dir);
    app.assess_db()?;
    Ok(app)
}
