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

pub fn evaluate_users_solution(conn: &Connection, base_dir: &str) -> Result<EvaluationResult> {
    let test_sql =
        read_to_string(format!("{base_dir}/{TEST_SQL_PATH}")).expect("Could not read test.sql");

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
