use rusqlite::Connection;
use std::path::PathBuf;

pub fn learn() {
    let conn = crate::db::schema::open_db();
    let history = parse_history();
    for cmd in &history {
        conn.execute(
            "INSERT OR IGNORE INTO commands (command, directory, timestamp) VALUES (?1, ?2, ?3)",
            rusqlite::params![cmd.command, cmd.directory, cmd.timestamp],
        ).ok();
    }
    println!("Learned {} commands from shell history", history.len());
}

struct HistoryEntry {
    command: String,
    directory: String,
    timestamp: i64,
}

fn parse_history() -> Vec<HistoryEntry> {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let mut entries = Vec::new();
    for hist_file in &[".bash_history", ".zsh_history"] {
        let path = home.join(hist_file);
        if let Ok(content) = std::fs::read_to_string(&path) {
            for line in content.lines() {
                let cmd = line.trim();
                if !cmd.is_empty() && !cmd.starts_with('#') {
                    entries.push(HistoryEntry {
                        command: cmd.to_string(),
                        directory: ".".to_string(),
                        timestamp: 0,
                    });
                }
            }
        }
    }
    entries
}
