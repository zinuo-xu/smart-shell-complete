use crate::db::schema;

pub fn predict() {
    let conn = schema::open_db();
    let cwd = std::env::current_dir()
        .ok()
        .and_then(|p| p.file_name().map(|s| s.to_string_lossy().into_owned()))
        .unwrap_or_default();

    let mut stmt = conn
        .prepare(
            "SELECT command, SUM(count) as score FROM commands
             WHERE directory = ?1 OR directory = '.'
             GROUP BY command ORDER BY score DESC LIMIT 10",
        )
        .expect("Failed to prepare query");

    let results: Vec<String> = stmt
        .query_map(rusqlite::params![cwd], |row| row.get::<_, String>(0))
        .expect("Query failed")
        .filter_map(|r| r.ok())
        .collect();

    if results.is_empty() {
        println!("No predictions yet. Run 'smart-shell-complete learn' first.");
        return;
    }

    println!("Top predictions:");
    for (i, cmd) in results.iter().enumerate() {
        println!("  {}. {}", i + 1, cmd);
    }
}

pub fn complete(prefix: &str) {
    let conn = schema::open_db();
    let pattern = format!("{}%", prefix);

    let mut stmt = conn
        .prepare(
            "SELECT command FROM commands
             WHERE command LIKE ?1
             GROUP BY command ORDER BY SUM(count) DESC LIMIT 10",
        )
        .expect("Failed to prepare query");

    let results: Vec<String> = stmt
        .query_map(rusqlite::params![pattern], |row| row.get::<_, String>(0))
        .expect("Query failed")
        .filter_map(|r| r.ok())
        .collect();

    if results.is_empty() {
        println!("No completions found for '{}'.", prefix);
        return;
    }

    for cmd in &results {
        println!("{}", cmd);
    }
}
