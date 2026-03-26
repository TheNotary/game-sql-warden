use glob::glob;
use log::{debug, error, trace};
use rusqlite::{Connection, OptionalExtension};
use std::collections::HashSet;
use std::fmt::{self, Debug};
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::{fs::read_to_string, path::Path};
use thiserror::Error;

use crate::{
    DB_PATH, MIGRATION_PATH, SOLUTION_PATH,
    evaluation::evaluate_users_solution,
    game::game_state::GameState,
    presenter::{db_created_string, instructions_string},
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
    debug!("handle_db_condition called with {}", state);
    use ActionToTake::*;
    use ChallengeState::*;

    Ok(match state {
        MissingDb(base_dir) => {
            migrate_challenge_db(&base_dir)?;
            NoAction(db_created_string(&base_dir))
        }
        NotAttempted(base_dir) => NoAction(instructions_string(&base_dir)),
        Attempted(base_dir, conn) => {
            let did_test_pass = evaluate_users_solution(&conn, &base_dir)?;

            if did_test_pass {
                LevelCleared("🏆 Tests passed!".into())
            } else {
                NoAction("Tests didn't go ok, try again!".into())
            }
        }
    })
}

pub fn assess_db_condition(base_dir: &str) -> Result<ChallengeState> {
    debug!("assess_db_condition was called. base_dir: {base_dir}");

    let db_path = &format!("{base_dir}/{DB_PATH}");
    if !Path::new(db_path).exists() {
        trace!("MissingDB");
        return Ok(ChallengeState::MissingDb(base_dir.to_string()));
    }

    let conn = Connection::open(db_path)?;

    if was_challenge_attempted(&conn)? {
        trace!("Attempted");
        return Ok(ChallengeState::Attempted(base_dir.to_string(), conn));
    }
    trace!("NotAttempted");
    Ok(ChallengeState::NotAttempted(base_dir.to_string()))
}

#[derive(Debug)]
pub enum ChallengeState {
    MissingDb(String),
    NotAttempted(String),
    Attempted(String, Connection),
}

impl fmt::Display for ChallengeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChallengeState::MissingDb(path) => write!(f, "MissingDb {path}"),
            ChallengeState::NotAttempted(path) => write!(f, "NotAttempted {path}"),
            ChallengeState::Attempted(path, _) => write!(f, "Attempted {path}"),
        }
    }
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

    #[error("Unable to pull data from App database")]
    FailedPullingFromAppDb,

    #[error("Something went wrong initializing the logger.")]
    LoggerInitFailure,
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

/// Remove ./database.db and ./challenges/**/database.db
pub fn reset_databases() {
    remove_if_exists("./database.db");

    for entry in glob("./challenges/**/database.db").expect("Invalid glob pattern") {
        match entry {
            Ok(path) => remove_if_exists(path.to_string_lossy().as_ref()),
            Err(e) => debug!("Glob error: {:?}", e),
        }
    }
}

/// Remove ./challenges/**/solution.sql
pub fn reset_solutions() {
    for entry in glob("./challenges/**/solution.sql").expect("Invalid glob pattern") {
        match entry {
            Ok(path) => remove_if_exists(path.to_string_lossy().as_ref()),
            Err(e) => debug!("Glob error: {:?}", e),
        }
    }
}

/// Removes a file if it exists, if it doesn't exist, just complains
/// # Panics
/// On permission issues and whatever else fs::remove_file might error on
fn remove_if_exists(path: &str) {
    match fs::remove_file(path) {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            debug!("File was missing, skipping delete of {}", path);
        }
        Err(e) => {
            panic!("Failed to delete {}: {:?}", path, e);
        }
    }
}

fn migrate_challenge_db(base_dir: &str) -> Result<()> {
    debug!("migrate_challenge_db was called with base_dir: {base_dir}");

    let migration_path_str = format!("{base_dir}/{MIGRATION_PATH}");
    if !Path::new(&migration_path_str).exists() {
        error!("MigrationFileMissing!: {migration_path_str}");
        return Err(ChallengeError::MigrationFileMissing(
            migration_path_str.to_string(),
        ));
    }

    let db_path = format!("{base_dir}/{DB_PATH}");
    execute_batch_sql_file(&migration_path_str, &db_path)
}

/// A helper function for executing a sql file against a db given paths to both.
///
/// # Panics
/// Probably
fn execute_batch_sql_file(path_to_sql: &str, path_to_db: &str) -> Result<()> {
    debug!("execute_batch_sql_file for {path_to_sql} against db at {path_to_db}");
    let sql_path = Path::new(path_to_sql);
    let sql_cmd = read_to_string(sql_path)?;

    let conn = Connection::open(path_to_db).unwrap();
    let status = conn.execute_batch(&sql_cmd);

    match status {
        Ok(_) => Ok(()),
        Err(_) => {
            error!("MigrationFailed");
            return Err(ChallengeError::MigrationFailed);
        }
    }
}

fn was_challenge_attempted(conn: &Connection) -> Result<bool> {
    let mut stmt =
        conn.prepare("SELECT name FROM sqlite_master WHERE type='view' AND name='solution';")?;

    let result = stmt.query_row([], |row| row.get::<usize, String>(0));

    Ok(result.optional()?.is_some())
}

pub fn setup_app_db() -> Result<()> {
    debug!("setup_app_db was called");
    debug!("DB_PATH: {}", DB_PATH);
    if Path::new(DB_PATH).exists() {
        debug!("App db path already exists, skipping migration for {DB_PATH}");
        return Ok(());
    }

    let sql_cmd = read_to_string(PathBuf::from("sql/migrate.sql"))?;
    execute_batch_sql_file(&sql_cmd, DB_PATH)
}

pub fn get_game_state_from_db() -> Result<GameState> {
    let conn = Connection::open(DB_PATH).unwrap();

    let player = get_player_from_db(&conn)?;
    let cleared_levels = get_cleared_levels_from_db(&conn)?;

    drop(conn);

    Ok(GameState::new(player, cleared_levels))
}

pub fn get_player_from_db(conn: &Connection) -> Result<(usize, usize)> {
    let mut stmt = conn.prepare("SELECT x, y FROM player WHERE id = :id")?;

    if let Ok(row) = stmt.query(&[(":id", "1")])?.next() {
        if let Some(row) = row {
            let x: usize = row.get(0)?;
            let y: usize = row.get(1)?;

            return Ok((x, y));
        }
    }

    Err(ChallengeError::FailedPullingFromAppDb)
}

pub fn get_cleared_levels_from_db(conn: &Connection) -> Result<HashSet<u32>> {
    let mut cleared_levels: HashSet<u32> = HashSet::new();

    let mut stmt = conn.prepare("SELECT id FROM cleared_levels")?;

    let cleared_stages = stmt.query_map([], |row| row.get::<usize, u32>(0))?;

    for stage_id in cleared_stages {
        if let Ok(stage_id) = stage_id {
            cleared_levels.insert(stage_id);
        }
    }

    Ok(cleared_levels)
}

pub fn clear_level_in_sqlite(stage_id: u32) -> Result<()> {
    let conn = Connection::open(DB_PATH).unwrap();

    let mut stmt = conn.prepare("INSERT INTO cleared_levels (id) VALUES (:id)")?;

    stmt.execute(&[(":id", &stage_id.to_string())])?;
    Ok(())
}

pub fn execute_solution(base_dir: &str) -> Result<()> {
    trace!("execute_solution called.  base_dir: {base_dir}");
    let db_path = &format!("{base_dir}/{DB_PATH}");

    if !Path::new(db_path.into()).exists() {
        trace!("Skipping execute_solution, db_path did not exist: {db_path}");
        return Ok(());
    }

    let solution_path_str = format!("{base_dir}/{SOLUTION_PATH}");

    execute_batch_sql_file(&solution_path_str, &db_path)
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
        assert!(matches!(result, ChallengeState::Attempted(..)));
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
