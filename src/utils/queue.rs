//! Utilities for interacting with the build queue

use postgres::Connection;
use error::Result;

pub fn add_crate_to_queue(conn: &Connection, name: &str, version: &str, priority: i32) -> Result<()> {
    conn.execute("INSERT INTO queue (name, version, priority) VALUES ($1, $2, $3)",
                      &[&name, &version, &priority])?;
    Ok(())
}
