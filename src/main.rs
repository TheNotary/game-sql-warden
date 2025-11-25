use rusqlite::{Connection, OptionalExtension};
use std::fs::read_to_string;
use std::path::Path;
use std::process::Command;
use thiserror::Error;

use crate::{
    app::App,
    evaluation::evaluate_users_solution,
    presenter::{db_created_string, evaluation_to_string, instructions_string},
    tui::tui_loop,
};

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

fn handle_db_condition(state: ChallengeState) -> Result<String> {
    match state {
        ChallengeState::MissingDb => {
            create_db()?;
            Ok(db_created_string())
        }
        ChallengeState::NotAttempted => Ok(instructions_string()),
        ChallengeState::Attempted(conn) => {
            let report = evaluate_users_solution(&conn)?;
            Ok(evaluation_to_string(&report))
        }
    }
}

pub enum ChallengeState {
    MissingDb,
    NotAttempted,
    Attempted(Connection),
}

#[derive(Error, Debug)]
pub enum ChallengeError {
    #[error("Migration file '{0}' not found")]
    MigrationFileMissing(String),

    #[error("sqlite3 command failed to apply migration")]
    MigrationFailed,

    #[error("Database operation failed: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, ChallengeError>;

fn assess_db_condition(db_path: &str) -> Result<ChallengeState> {
    if !Path::new(db_path).exists() {
        return Ok(ChallengeState::MissingDb);
    }

    let conn = Connection::open(db_path)?;

    if was_challenge_attempted(&conn)? {
        return Ok(ChallengeState::Attempted(conn));
    }
    Ok(ChallengeState::NotAttempted)
}

fn delete_db_file(db_path_str: &str) -> Result<()> {
    let db_path = Path::new(db_path_str);
    if db_path.exists() {
        std::fs::remove_file(db_path)?;
    }
    Ok(())
}

fn create_db() -> Result<()> {
    if !Path::new(MIGRATION_PATH).exists() {
        return Err(ChallengeError::MigrationFileMissing(
            MIGRATION_PATH.to_string(),
        ));
    }

    let status = Command::new("sqlite3")
        .arg(DB_PATH)
        .arg(format!(".read {}", MIGRATION_PATH))
        .status()?;

    if !status.success() {
        return Err(ChallengeError::MigrationFailed);
    }

    Ok(())
}

fn was_challenge_attempted(conn: &Connection) -> Result<bool> {
    let mut stmt = conn.prepare(
        "SELECT name FROM sqlite_master WHERE type='view' AND name='strongest_monsters';",
    )?;

    Ok(stmt
        .query_row([], |row| row.get::<usize, String>(0))
        .optional()?
        .is_some())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use tempfile::TempDir;

    //////////////////////////////
    // Test assess_db_condition //
    //////////////////////////////

    // Helper to set up a temporary database directory
    fn setup_temp_db_dir() -> TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    fn test_missing_database() {
        let temp_dir = setup_temp_db_dir();
        let non_existent_path = temp_dir.path().join("nonexistent.db");

        // Temporarily override DB_PATH for this test
        // Note: This assumes DB_PATH can be mocked or you use dependency injection
        let result = assess_db_condition(non_existent_path.to_str().unwrap()).unwrap();

        assert!(matches!(result, ChallengeState::MissingDb));
    }

    #[test]
    fn test_not_attempted() {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        // populate_db(create_monster_table_sql)
        let conn = Connection::open(&db_path).unwrap();
        conn.execute(
            "CREATE TABLE monsters (id INTEGER, name TEXT, strength INTEGER)",
            [],
        )
        .unwrap();
        drop(conn);

        let result = assess_db_condition(db_path.to_str().unwrap()).unwrap();
        assert!(matches!(result, ChallengeState::NotAttempted));
    }

    #[test]
    fn test_attempted() {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        // setup_db(create_incorrect_solution)
        let conn = Connection::open(&db_path).unwrap();
        conn.execute("CREATE TABLE monsters (id INTEGER)", [])
            .unwrap();
        conn.execute(
            "CREATE VIEW strongest_monsters AS SELECT * FROM monsters",
            [],
        )
        .unwrap();
        drop(conn);

        let result = assess_db_condition(db_path.to_str().unwrap()).unwrap();
        assert!(matches!(result, ChallengeState::Attempted(_)));
    }

    //////////////////////////////////
    // Test was_challenge_attempted //
    //////////////////////////////////

    #[test]
    fn test_challenge_not_attempted() {
        // Setup: Create an in-memory database without the view
        let conn = Connection::open_in_memory().unwrap();

        // Create some basic table structure if needed
        conn.execute(
            "CREATE TABLE monsters (id INTEGER PRIMARY KEY, name TEXT, strength INTEGER)",
            [],
        )
        .unwrap();

        // Test: Should return false when view doesn't exist
        let result = was_challenge_attempted(&conn).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_challenge_attempted() {
        // Setup: Create an in-memory database with the view
        let conn = Connection::open_in_memory().unwrap();

        // Create prerequisite table
        conn.execute(
            "CREATE TABLE monsters (id INTEGER PRIMARY KEY, name TEXT, strength INTEGER)",
            [],
        )
        .unwrap();

        // Create the view that indicates challenge was attempted
        conn.execute(
            "CREATE VIEW strongest_monsters AS SELECT * FROM monsters ORDER BY strength DESC",
            [],
        )
        .unwrap();

        // Test: Should return true when view exists
        let result = was_challenge_attempted(&conn).unwrap();
        assert!(result);
    }

    #[test]
    fn test_handles_empty_database() {
        // Setup: Completely empty database
        let conn = Connection::open_in_memory().unwrap();

        // Test: Should return false without errors
        let result = was_challenge_attempted(&conn).unwrap();
        assert!(!result);
    }
}
