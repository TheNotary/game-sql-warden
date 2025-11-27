use rusqlite::{Connection, OptionalExtension};
use std::fs::File;
use std::process::Command;
use std::{fs::read_to_string, path::Path};
use thiserror::Error;

use crate::{
    DB_PATH, MIGRATION_PATH,
    evaluation::evaluate_users_solution,
    presenter::{db_created_string, evaluation_to_string, instructions_string},
};

pub enum ActionToTake {
    NoAction(String),
    LevelCleared(String),
}

impl ActionToTake {
    pub(crate) fn into_output(&self) -> String {
        String::from(match self {
            Self::LevelCleared(output) => output,
            Self::NoAction(output) => output,
        })
    }
}

pub fn handle_db_condition(state: ChallengeState) -> Result<ActionToTake> {
    use ActionToTake::*;
    use ChallengeState::*;

    Ok(match state {
        MissingDb(base_dir) => {
            create_db(&base_dir)?;
            NoAction(db_created_string(&base_dir))
        }
        NotAttempted(base_dir) => NoAction(instructions_string(&base_dir)),
        Attempted(conn) => {
            let report = evaluate_users_solution(&conn)?;

            if report.all_correct {
                LevelCleared(evaluation_to_string(&report))
            } else {
                NoAction(evaluation_to_string(&report))
            }
        }
    })
}

pub fn assess_db_condition(base_dir: &str) -> Result<ChallengeState> {
    let db_path = &format!("{base_dir}/{DB_PATH}");
    if !Path::new(db_path).exists() {
        return Ok(ChallengeState::MissingDb(base_dir.to_string()));
    }

    let conn = Connection::open(db_path)?;

    if was_challenge_attempted(&conn)? {
        return Ok(ChallengeState::Attempted(conn));
    }
    Ok(ChallengeState::NotAttempted(base_dir.to_string()))
}

pub enum ChallengeState {
    MissingDb(String),
    NotAttempted(String),
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

    #[error("Unable to parse challenge directory name: {0}")]
    InvalidChallengeDir(String),
}

pub type Result<T> = std::result::Result<T, ChallengeError>;

pub fn delete_db_file(db_path_str: &str) -> Result<()> {
    let db_path = Path::new(db_path_str);
    if db_path.exists() {
        std::fs::remove_file(db_path)?;
    }
    Ok(())
}

pub fn read_solution_file(solution_path: &str) -> String {
    if !Path::new(solution_path).exists() {
        File::create(solution_path).expect(&format!("Couldn't create file {solution_path}"));
    }
    read_to_string(solution_path).expect(&format!("Unable to read {solution_path}."))
}

pub fn read_instructions_file(instruction_path: &str) -> String {
    read_to_string(instruction_path).expect(&format!("Unable to read {instruction_path}."))
}

pub fn read_lore_file(lore_path: &str) -> String {
    read_to_string(lore_path).expect(&format!("Unable to read {lore_path}."))
}

pub fn read_challenge_name(name_path: &str) -> String {
    read_to_string(name_path).expect(&format!("Unable to read {name_path}."))
}

fn create_db(base_dir: &str) -> Result<()> {
    let migration_path = format!("{base_dir}/{MIGRATION_PATH}");
    if !Path::new(&migration_path).exists() {
        return Err(ChallengeError::MigrationFileMissing(
            migration_path.to_string(),
        ));
    }

    let status = Command::new("sqlite3")
        .arg(format!("{base_dir}/{DB_PATH}"))
        .arg(format!(".read {}", migration_path))
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
        let _blah = "hi".to_string();

        // Temporarily override DB_PATH for this test
        // Note: This assumes DB_PATH can be mocked or you use dependency injection
        let result = assess_db_condition(non_existent_path.to_str().unwrap()).unwrap();

        assert!(matches!(result, ChallengeState::MissingDb(_blah)));
    }

    #[test]
    fn test_not_attempted() {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let _blah = "ignored".to_string();

        // populate_db(create_monster_table_sql)
        let conn = Connection::open(&db_path).unwrap();
        conn.execute(
            "CREATE TABLE monsters (id INTEGER, name TEXT, strength INTEGER)",
            [],
        )
        .unwrap();
        drop(conn);

        let result = assess_db_condition(db_path.to_str().unwrap()).unwrap();
        assert!(matches!(result, ChallengeState::NotAttempted(_blah)));
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
