use rusqlite::Result;

use crate::DB_PATH;
use crate::evaluation::EvaluationResult;

pub fn print_evaluation(result: &EvaluationResult) -> Result<()> {
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
    Ok(())
}

pub fn print_db_created_note() -> Result<()> {
    println!("✅ Dungeon constructed! database.db is ready.");
    println!("To explore the dungeon manually:");
    println!("  sqlite3 {DB_PATH}");
    println!("Inside SQLite, view the schema with:");
    println!("  .schema");
    Ok(())
}

pub fn print_instruction_what_to_do() -> Result<()> {
    println!("🧙 The Warden whispers: You have not yet attempted the challenge.");
    println!("The dungeon is ready. Create your solution as:");
    println!("  CREATE VIEW strongest_monsters AS ... ;");
    println!("");
    println!("To inspect the database schema:");
    println!("  sqlite3 {DB_PATH}");
    println!("  .schema");
    Ok(())
}
