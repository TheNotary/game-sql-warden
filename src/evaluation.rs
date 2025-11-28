use rusqlite::{Connection, Result};
use std::fs::read_to_string;

use crate::TEST_SQL_PATH;

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

pub fn evaluate_users_solution(conn: &Connection, base_dir: &str) -> Result<bool> {
    let test_sql_path = format!("{base_dir}/{TEST_SQL_PATH}");
    let test_sql = read_to_string(test_sql_path).expect("Could not read test.sql");

    let mut stmt = conn.prepare(&test_sql)?;

    let result: String = stmt.query_row([], |row| row.get(0))?;

    Ok(result == "PASS")
}
