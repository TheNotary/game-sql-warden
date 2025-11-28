use crate::DB_PATH;
use std::fmt::{self, Write};

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

pub fn db_created_string(base_dir: &str) -> String {
    let mut out = String::new();

    push_line!(
        out,
        "🧱 {base_dir}/{DB_PATH} not found — constructing the Cubical Dungeon..."
    );
    push_line!(out, "");
    push_line!(out, "✅ Dungeon constructed! database.db is ready.");
    push_line!(out, "To explore the dungeon manually:");
    push_line!(out, "  sqlite3 {base_dir}/{DB_PATH}");
    push_line!(out, "Inside SQLite, view the schema with:");
    push_line!(out, "  .schema");

    out
}

pub fn instructions_string(base_dir: &str) -> String {
    let mut out = String::new();

    push_line!(
        out,
        "🧙 The Warden whispers: You have not yet attempted the challenge."
    );
    push_line!(out, "The dungeon is ready. Create your solution as:");
    push_line!(out, "  CREATE VIEW strongest_monsters AS ... ;");
    push_line!(out, "");
    push_line!(out, "To inspect the database schema:");
    push_line!(out, "  sqlite3 {base_dir}/{DB_PATH}");
    push_line!(out, "  .schema");

    out
}
