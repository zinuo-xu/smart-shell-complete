use rusqlite::Connection;
use std::path::PathBuf;

pub fn open_db() -> Connection {
    let db_path = db_path();
    let conn = Connection::open(&db_path).expect("Failed to open database");
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS commands (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            command TEXT NOT NULL,
            directory TEXT,
            timestamp INTEGER,
            count INTEGER DEFAULT 1
        );
        CREATE TABLE IF NOT EXISTS chains (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            prev_command TEXT NOT NULL,
            next_command TEXT NOT NULL,
            count INTEGER DEFAULT 1
        );"
    ).expect("Failed to create tables");
    conn
}

fn db_path() -> PathBuf {
    let mut path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("smart-shell-complete");
    std::fs::create_dir_all(&path).ok();
    path.push("commands.db");
    path
}
