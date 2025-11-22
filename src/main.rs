use rusqlite::{Connection, OptionalExtension};
use std::path::Path;
use std::process::Command;
use thiserror::Error;

use crate::{
    evaluation::evaluate_users_solution, presenter::print_db_created_note,
    presenter::print_evaluation, presenter::print_instruction_what_to_do,
};

mod evaluation;
mod presenter;

pub static DB_PATH: &str = "database.db";
pub static MIGRATION_PATH: &str = "migration.sql";
pub static TEST_SQL_PATH: &str = "test.sql";

fn main() -> Result<()> {
    match assess_db_condition()? {
        ChallengeState::MissingDb => {
            println!("🧱 {DB_PATH} not found — constructing the Cubical Dungeon...");
            create_db()?;
            print_db_created_note();
            Ok(())
        }
        ChallengeState::NotAttempted => {
            print_instruction_what_to_do();
            Ok(())
        }
        ChallengeState::Attempted(conn) => {
            let report = evaluate_users_solution(&conn)?;
            print_evaluation(&report);
            Ok(())
        }
    }
}

enum ChallengeState {
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

type Result<T> = std::result::Result<T, ChallengeError>;

fn assess_db_condition() -> Result<ChallengeState> {
    if !Path::new(DB_PATH).exists() {
        return Ok(ChallengeState::MissingDb);
    }

    let conn = Connection::open(DB_PATH)?;

    if was_challenge_attempted(&conn)? {
        return Ok(ChallengeState::Attempted(conn));
    }
    Ok(ChallengeState::NotAttempted)
}

fn create_db() -> Result<()> {
    if !Path::new(MIGRATION_PATH).exists() {
        eprintln!("❌ {MIGRATION_PATH} missing — cannot build {DB_PATH}");
        return Err(ChallengeError::MigrationFileMissing(
            MIGRATION_PATH.to_string(),
        ));
    }

    let status = Command::new("sqlite3")
        .arg(DB_PATH)
        .arg(format!(".read {}", MIGRATION_PATH))
        .status()?;

    if !status.success() {
        eprintln!("❌ sqlite3 failed to apply migration");
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
