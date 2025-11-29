use log::info;

use crate::{
    api::{ChallengeError, Result, delete_db_file, get_game_state_from_db, setup_app_db},
    app::{App, Stage},
    tui::tui_loop,
};

mod api;
mod app;
mod evaluation;
mod game;
mod presenter;
mod tui;
mod views;

pub static DB_PATH: &str = "database.db";
pub static NAME_PATH: &str = "00_name.txt";
pub static LORE_PATH: &str = "01_lore.md";
pub static INSTRUCTIONS_PATH: &str = "02_instructions.md";
pub static MIGRATION_PATH: &str = "03_migration.sql";
pub static TEST_SQL_PATH: &str = "04_test.sql";
pub static SOLUTION_PATH: &str = "solution.sql";

fn main() -> Result<()> {
    setup_logger()?;

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
    setup_app_db()?;

    let game_state = get_game_state_from_db()?;
    let stage = Stage::from_dir(challenge_dir);
    let mut app = App::new(stage, game_state);

    app.assess_db()?;
    Ok(app)
}

fn setup_logger() -> Result<()> {
    match fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file("output.log")?)
        .apply()
    {
        Ok(_) => {
            info!("################");
            info!("#    BOOTED    #");
            info!("################");
            Ok(())
        }
        Err(_) => Err(ChallengeError::LoggerInitFailure),
    }
}
