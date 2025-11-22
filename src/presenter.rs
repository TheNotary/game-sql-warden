use crate::DB_PATH;
use crate::evaluation::EvaluationResult;
use std::fmt::{self, Write};

// FIXME: Move to it's own crate
pub trait PushLine {
    fn push_line(&mut self, args: fmt::Arguments);
}

impl PushLine for String {
    fn push_line(&mut self, args: fmt::Arguments) {
        self.write_fmt(args).unwrap();
        self.push('\n');
    }
}

#[macro_export]
macro_rules! push_line {
    ($string:expr, $($arg:tt)*) => {{
        $string.push_line(format_args!($($arg)*));
    }};
}

pub fn evaluation_to_string(result: &EvaluationResult) -> String {
    let mut out = String::new();

    push_line!(out, "🔍 Attempt detected! Evaluating your solution...");
    push_line!(out, "");
    push_line!(out, "📊 Test Results:");

    for r in &result.rows {
        if r.is_correct {
            push_line!(
                out,
                " cube {} → monster {} ✔ correct",
                r.cube_id,
                r.monster_id
            );
        } else {
            push_line!(
                out,
                " cube {} → monster {} ✘ incorrect",
                r.cube_id,
                r.monster_id
            );
        }
    }

    push_line!(out, "");

    if result.all_correct {
        push_line!(
            out,
            "🏆 **You have mastered the Cubical Dungeon’s first trial!**"
        );
    } else {
        push_line!(out, "❌ Some answers were incorrect.");
        push_line!(out, "The Warden mutters: 'Refine your query, wanderer.'");
    }

    out
}

pub fn db_created_string() -> String {
    let mut out = String::new();

    push_line!(
        out,
        "🧱 {DB_PATH} not found — constructing the Cubical Dungeon..."
    );
    push_line!(out, "");
    push_line!(out, "✅ Dungeon constructed! database.db is ready.");
    push_line!(out, "To explore the dungeon manually:");
    push_line!(out, "  sqlite3 {DB_PATH}");
    push_line!(out, "Inside SQLite, view the schema with:");
    push_line!(out, "  .schema");

    out
}

pub fn instructions_string() -> String {
    let mut out = String::new();

    push_line!(
        out,
        "🧙 The Warden whispers: You have not yet attempted the challenge."
    );
    push_line!(out, "The dungeon is ready. Create your solution as:");
    push_line!(out, "  CREATE VIEW strongest_monsters AS ... ;");
    push_line!(out, "");
    push_line!(out, "To inspect the database schema:");
    push_line!(out, "  sqlite3 {DB_PATH}");
    push_line!(out, "  .schema");

    out
}
