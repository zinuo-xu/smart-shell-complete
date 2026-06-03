use crate::db::schema;

pub fn show_stats() {
    let conn = schema::open_db();
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM commands", [], |row| row.get(0)
    ).unwrap_or(0);
    let chains: i64 = conn.query_row(
        "SELECT COUNT(*) FROM chains", [], |row| row.get(0)
    ).unwrap_or(0);
    println!("Commands learned: {}", count);
    println!("Chains detected:  {}", chains);
}
