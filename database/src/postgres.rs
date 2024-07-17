use postgres::{Client, Error, NoTls};

/// Create a new database.
///
/// # Arguments
///
/// * `connection_string` - A connection string to the PostgreSQL server.
/// * `db_name` - The name of the database to create.
///
/// # Returns
///
/// A `Result` indicating whether the operation was successful.
///
/// # Example
///
/// ```ignore
/// use database::create_db;
///
/// let db_name = "test_db";
/// let connection_string = "postgresql://postgres:@localhost";
///
/// let result = create_db(connection_string, db_name);
/// assert!(result.is_ok());
/// ```
pub fn create_db(connection_string: &str, db_name: &str) -> Result<(), Error> {
    let mut client = Client::connect(connection_string, NoTls)?;

    match client.batch_execute(&format!("CREATE DATABASE {}", db_name)) {
        Ok(_) => println!("Database created successfully."),
        Err(e) => {
            if e.as_db_error()
                .map_or(false, |e| e.code().code() == "42P04")
            {
                println!("Database already exists.");
            } else {
                return Err(e);
            }
        }
    }

    Ok(())
}

/// Delete an existing database.
///
/// # Arguments
///
/// * `connection_string` - A connection string to the PostgreSQL server.
/// * `db_name` - The name of the database to delete.
///
/// # Returns
///
/// A `Result` indicating whether the operation was successful.
///
/// # Example
///
/// ```ignore
/// use database::delete_db;
///
/// let db_name = "test_db";
/// let connection_string = "postgresql://postgres:@localhost";
///
/// let result = delete_db(connection_string, db_name);
/// assert!(result.is_ok());
/// ```
pub fn delete_db(connection_string: &str, db_name: &str) -> Result<(), Error> {
    let mut client = Client::connect(connection_string, NoTls)?;

    match client.batch_execute(&format!("DROP DATABASE {}", db_name)) {
        Ok(_) => println!("Database deleted successfully."),
        Err(e) => {
            if e.as_db_error()
                .map_or(false, |e| e.code().code() == "3D000")
            {
                println!("Database does not exist.");
            } else {
                return Err(e);
            }
        }
    }

    Ok(())
}

/// Add tables to a database.
///
/// # Arguments
///
/// * `connection_string` - A connection string to the PostgreSQL server.
/// * `tables` - A vector of strings representing the tables to create.
///
/// # Returns
///
/// A `Result` indicating whether the operation was successful.
///
/// # Example
///
/// ```ignore
/// use database::add_tables;
///
/// let tables: Vec<&str> = vec![
///     "author (
///       id              SERIAL PRIMARY KEY,
///       name            VARCHAR NOT NULL,
///       country         VARCHAR NOT NULL
///     )",
///     "book  (
///       id              SERIAL PRIMARY KEY,
///       title           VARCHAR NOT NULL,
///       author_id       INTEGER NOT NULL REFERENCES author
///     )",
///   ];
///
/// let connection_string = "postgresql://postgres:@localhost/test_db";
///
/// let result = add_tables(connection_string, tables);
/// assert!(result.is_ok());
/// ```
pub fn add_tables(connection_string: &str, tables: Vec<&str>) -> Result<(), Error> {
    let mut client = Client::connect(connection_string, NoTls)?;

    for table in tables {
        client.batch_execute(&format!("CREATE TABLE IF NOT EXISTS {}", table))?;
    }

    Ok(())
}
