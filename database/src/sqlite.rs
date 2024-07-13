use rusqlite::{params, Connection, Params, Result};
use shared::Cat;
use std::{collections::HashMap, error::Error};

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
pub fn create_sqlite_cats_database(database: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(database)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS cat_colors (
             id INTEGER PRIMARY KEY,
             name TEXT NOT NULL UNIQUE
         )",
        params![],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS cats (
             id INTEGER PRIMARY KEY,
             name TEXT NOT NULL,
             color_id INTEGER NOT NULL REFERENCES cat_colors(id)
         )",
        params![],
    )?;

    Ok(())
}

/// Delete the cat_colors and cats tables from the SQLite database.
///
/// # Arguments
///
/// * `database` - The name of the SQLite database.
///
/// # Examples
///
/// ```ignore
/// use database::delete_cats_database;
///
/// let database = "test.db";
/// let result = delete_cats_database(database);
/// assert!(result.is_ok());
/// ```
pub fn delete_cats_database(database: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(database)?;

    conn.execute("DROP TABLE IF EXISTS cats;", params![])?;
    conn.execute("DROP TABLE IF EXISTS cat_colors;", params![])?;

    create_sqlite_cats_database(database)
}

pub fn insert_select_cats(database: &str, cats: &Vec<Cat>) -> Result<Vec<Cat>, Box<dyn Error>> {
    let conn = Connection::open(database)?;

    let mut cat_colors = HashMap::new();

    for cat in cats {
        let color = cat.color.to_string();
        let name = cat.name.to_string();
        let entry = cat_colors.entry(color).or_insert(vec![]);
        entry.push(name);
    }

    for (color, catnames) in &cat_colors {
        conn.execute(
            "INSERT INTO cat_colors (name) VALUES (?1)",
            &[&color.to_string()],
        )?;
        let last_id: String = conn.last_insert_rowid().to_string();

        for cat in catnames {
            conn.execute(
                "INSERT INTO cats (name, color_id) VALUES (?1, ?2)",
                &[&cat.to_string(), &last_id],
            )?;
        }
    }
    let mut stmt = conn.prepare(
        "SELECT c.name, cc.name FROM cats c
         INNER JOIN cat_colors cc
         ON cc.id = c.color_id;",
    )?;

    let cats_result: Result<Vec<Cat>, rusqlite::Error> = stmt
        .query_map(params![], |row| {
            Ok(Cat {
                name: row.get(0)?,
                color: row.get(1)?,
            })
        })?
        .collect();

    cats_result.map_err(|e| Box::new(e) as Box<dyn Error>)
}

/// SQLite transaction types.
///
/// # Commit
///
/// Commit the transaction.
///
/// # Rollback
///
/// Rollback the transaction.
///
/// # Examples
///
/// ```
/// use database::TransactionType;
///
/// let tx_type = TransactionType::Commit;
/// ```
pub enum TransactionType {
    Commit,
    Rollback,
}

/// Submit a database transaction.
///
/// # Arguments
///
/// * `database` - The name of the SQLite database.
/// * `query` - The SQL query.
/// * `params` - The query parameters.
/// * `tx_type` - The transaction type.
///
/// # Examples
///
/// ```ignore
/// use database::*;
/// use rusqlite::params;
///
/// let database = "test.db";
/// let quey_no_params = "DELETE FROM cat_colors";
/// let transaction = submit_db_transaction(
///    database,
///    &quey_no_params,
///    params![],
///    TransactionType::Commit,
/// );
///
/// assert!(transaction.is_ok());
/// ```
pub fn submit_db_transaction<P: Params>(
    database: &str,
    query: &str,
    params: P,
    tx_type: TransactionType,
) -> Result<(), Box<dyn Error>> {
    let mut conn = Connection::open(database).unwrap();
    let tx = conn.transaction()?;

    tx.execute(query, params)?;

    let result = match tx_type {
        TransactionType::Commit => tx.commit(),
        TransactionType::Rollback => tx.rollback(),
    };

    result.map_err(|e| Box::new(e) as Box<dyn Error>)
}
