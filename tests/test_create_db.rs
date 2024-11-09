use rusqlite::{Connection, Result};
use std::fs;

///
/// Tests that CREATE TABLE statements work and there are no syntax errors
/// on the sql file
/// 
#[test]
fn test_load_sql_from_file() -> Result<()> {
    let sql_path = std::env::var("SQL_FILE_PATH").unwrap_or_else(|_| {
        format!("{}/doc/database.sql", env!("CARGO_MANIFEST_DIR"))
    });

    let sql_conn = Connection::open_in_memory()?;
    let result = fs::read_to_string(sql_path);
    
    let sql = match result {
        Ok(content) => {
            println!("SQL file loaded successfully.");
            content
        },
        Err(e) => {
            eprintln!("Failed to read SQL file: {}", e);
            return Err(rusqlite::Error::InvalidQuery); 
        },
    };

    if let Err(e) = sql_conn.execute_batch(&sql) {
        eprintln!("Create stmt fail: {}", e);
        return Err(e);
    } 
    else {
        println!("SQL file create stmt inserted OK.");
    }
    
    println!("SQL statements executed successfully.");
    Ok(())
}
