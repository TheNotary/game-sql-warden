use rusqlite::{Connection, OptionalExtension, Result};
use std::fs;
use std::process::Command;

static DB_PATH: &str = "database.db";
static MIGRATION_PATH: &str = "migration.sql";
static TEST_SQL_PATH: &str = "test.sql";

#[derive(PartialEq)]
enum ChallengeState {
    MissingDb,
    NotAttempted,
    Attempted,
}

fn main() -> Result<()> {
    match assess_environment()? {
        ChallengeState::MissingDb => {
            println!("🧱 {DB_PATH} not found — constructing the Cubical Dungeon...");
            create_db()?;
            print_db_created_note()
        }
        ChallengeState::NotAttempted => print_instruction_what_to_do(),
        ChallengeState::Attempted => {
            let conn = Connection::open(DB_PATH)?;
            if let Ok(report) = evaluate_users_solution(&conn) {
                print_evaluation(&report);
            }
            Ok(())
        }
    }
}

pub fn print_evaluation(result: &EvaluationResult) {
    println!("🔍 Attempt detected! Evaluating your solution...");
    println!("\n📊 Test Results:");

    for r in &result.rows {
        if r.is_correct {
            println!(" cube {} → monster {} ✔ correct", r.cube_id, r.monster_id);
        } else {
            println!(" cube {} → monster {} ✘ incorrect", r.cube_id, r.monster_id);
        }
    }

    println!();

    if result.all_correct {
        println!("🏆 **You have mastered the Cubical Dungeon’s first trial!**");
    } else {
        println!("❌ Some answers were incorrect.");
        println!("The Warden mutters: 'Refine your query, wanderer.'");
    }
}

fn print_db_created_note() -> Result<()> {
    println!("✅ Dungeon constructed! database.db is ready.");
    println!("To explore the dungeon manually:");
    println!("  sqlite3 {DB_PATH}");
    println!("Inside SQLite, view the schema with:");
    println!("  .schema");
    Ok(())
}

fn print_instruction_what_to_do() -> Result<()> {
    println!("🧙 The Warden whispers: You have not yet attempted the challenge.");
    println!("The dungeon is ready. Create your solution as:");
    println!("  CREATE VIEW strongest_monsters AS ... ;");
    println!("");
    println!("To inspect the database schema:");
    println!("  sqlite3 {DB_PATH}");
    println!("  .schema");
    Ok(())
}

fn assess_environment() -> Result<ChallengeState> {
    if !std::path::Path::new(DB_PATH).exists() {
        return Ok(ChallengeState::MissingDb);
    }

    let conn = Connection::open(DB_PATH)?;

    let attempted = was_challenge_attempted(&conn)?;
    conn.close().expect("SQLITE3 closed bad I guess");

    if attempted {
        return Ok(ChallengeState::Attempted);
    }
    Ok(ChallengeState::NotAttempted)
}

fn create_db() -> Result<()> {
    if !std::path::Path::new(MIGRATION_PATH).exists() {
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

    return Ok(());
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

fn evaluate_users_solution_bad(conn: &Connection) -> Result<()> {
    println!("🔍 Attempt detected! Evaluating your solution...");

    let test_sql = fs::read_to_string(TEST_SQL_PATH).expect("Could not read test.sql");

    let mut stmt = conn.prepare(&test_sql)?;

    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?, // cube_id
            row.get::<_, i64>(1)?, // monster_id
            row.get::<_, i64>(2)?, // is_correct
        ))
    })?;

    let mut all_correct = true;
    println!("\n📊 Test Results:");
    for row in rows {
        let (cube_id, monster_id, is_correct) = row?;
        if is_correct == 1 {
            println!(" cube {} → monster {} ✔ correct", cube_id, monster_id);
        } else {
            println!(" cube {} → monster {} ✘ incorrect", cube_id, monster_id);
            all_correct = false;
        }
    }

    println!();
    if all_correct {
        println!("🏆 **You have mastered the Cubical Dungeon’s first trial!**");
    } else {
        println!("❌ Some answers were incorrect.");
        println!("The Warden mutters: 'Refine your query, wanderer.'");
    }

    Ok(())
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

#[derive(Debug, Clone)]
pub struct EvaluationRow {
    pub cube_id: i64,
    pub monster_id: i64,
    pub is_correct: bool,
}

#[derive(Debug, Clone)]
pub struct EvaluationResult {
    pub rows: Vec<EvaluationRow>,
    pub all_correct: bool,
}
