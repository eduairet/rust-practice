use rusqlite::{params, Connection, Result};
use std::error::Error;

/// Create a SQLite database with two tables: cat_colors and cats.
///
/// # Arguments
///
/// * `database` - The name of the SQLite database.
///
/// # Examples
///
/// ```ignore
/// use database::*;
///
/// let database = "test.db";
/// let result = create_sqlite_database(database);
/// assert!(result.is_ok());
/// ```
pub fn create_sqlite_database(database: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(database)?;

    conn.execute(
        "create table if not exists cat_colors (
             id integer primary key,
             name text not null unique
         )",
        params![],
    )?;

    conn.execute(
        "create table if not exists cats (
             id integer primary key,
             name text not null,
             color_id integer not null references cat_colors(id)
         )",
        params![],
    )?;

    Ok(())
}
