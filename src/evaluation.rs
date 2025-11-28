use log::{debug, trace};
use rusqlite::{Connection, Result};
use std::fs::read_to_string;

use crate::TEST_SQL_PATH;

pub fn evaluate_users_solution(conn: &Connection, base_dir: &str) -> Result<bool> {
    let test_sql_path = format!("{base_dir}/{TEST_SQL_PATH}");

    debug!("evaluate_users_solution called, test_sql_path: {test_sql_path}");
    let test_sql = read_to_string(test_sql_path).expect("Could not read test.sql");

    let mut stmt = conn.prepare(&test_sql)?;
    trace!("prepare complete");

    let result: String = stmt.query_row([], |row| row.get(0))?;
    trace!("query complete");

    Ok(result == "PASS")
}
