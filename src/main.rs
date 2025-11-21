use rusqlite::{Connection, OptionalExtension, Result};
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::{
    evaluation::{EvaluationResult, EvaluationRow},
    presenter::print_db_created_note,
    presenter::print_evaluation,
    presenter::print_instruction_what_to_do,
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
            return print_db_created_note();
        }
        ChallengeState::NotAttempted => print_instruction_what_to_do(),
        ChallengeState::Attempted(conn) => {
            let report = evaluate_users_solution(&conn)?;
            print_evaluation(&report)
        }
    }
}

enum ChallengeState {
    MissingDb,
    NotAttempted,
    Attempted(Connection),
}

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
        return get_ret("Migration file missing");
    }

    let status = Command::new("sqlite3")
        .arg(DB_PATH)
        .arg(format!(".read {}", MIGRATION_PATH))
        .status()
        .expect("failed to run sqlite3");

    if !status.success() {
        eprintln!("❌ sqlite3 failed to apply migration");
        return get_ret("sqlite3 failed to apply migration");
    }

    Ok(())
}

fn was_challenge_attempted(conn: &Connection) -> Result<bool> {
    let mut stmt = conn.prepare(
        "SELECT name FROM sqlite_master WHERE type='view' AND name='strongest_monsters';",
    )?;

    if let Ok(attempt) = stmt
        .query_row([], |row| row.get::<usize, String>(0))
        .optional()
    {
        match attempt {
            None => return Ok(false),
            Some(_) => return Ok(true),
        }
    }

    get_ret("The SQL was invalid apparently...")
}

pub fn evaluate_users_solution(conn: &Connection) -> Result<EvaluationResult> {
    let test_sql = fs::read_to_string(TEST_SQL_PATH).expect("Could not read test.sql");

    let mut stmt = conn.prepare(&test_sql)?;
    let rows_iter = stmt.query_map([], |row| {
        Ok(EvaluationRow {
            cube_id: row.get(0)?,
            monster_id: row.get(1)?,
            is_correct: row.get::<_, i64>(2)? == 1,
        })
    })?;

    let mut rows = Vec::new();
    let mut all_correct = true;

    for row in rows_iter {
        let row = row?;
        if !row.is_correct {
            all_correct = false;
        }
        rows.push(row);
    }

    Ok(EvaluationResult { rows, all_correct })
}

fn get_ret<T>(msg: &str) -> std::result::Result<T, rusqlite::Error> {
    Err(rusqlite::Error::SqliteFailure(
        rusqlite::ffi::Error::new(0),
        Some(msg.into()),
    ))
}
