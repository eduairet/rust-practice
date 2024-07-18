use csv::ReaderBuilder;
use postgres::{Client, Error, NoTls};
use std::fs::File;

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

/// Insert data into a database.
///
/// # Arguments
///
/// * `connection_string` - A connection string to the PostgreSQL server.
/// * `data` - A vector of strings representing the data to insert.
///
/// # Returns
///
/// A `Result` indicating whether the operation was successful.
///
/// # Example
///
/// ```ignore
/// use database::{create_db, add_tables, insert_data};
///
/// let data: Vec<&str> = vec![
///    "INSERT INTO author (name, country) VALUES ('George R. R. Martin', 'United States')",
///    "INSERT INTO author (name, country) VALUES ('J. R. R. Tolkien', 'United Kingdom')",
/// ];
///
/// let connection_string = "postgresql://postgres:@localhost";
/// let db = "test_db";
///
/// create_db(connection_string, db).unwrap();
///
/// let connection_string = format!("{}/{}", connection_string, db);
/// let tables: Vec<&str> = vec![
///   "author (
///      id              SERIAL PRIMARY KEY,
///      name            VARCHAR NOT NULL,
///      country         VARCHAR NOT NULL
///   )",
/// ];
///
/// add_tables(&connection_string, tables).unwrap();
///
/// let data: Vec<&str> = vec![
///   "author (name, country) VALUES ('George R. R. Martin', 'United States')",
///   "author (name, country) VALUES ('J. R. R. Tolkien', 'United Kingdom')",
/// ];
///
/// let result = insert_data(&connection_string, data);
/// assert!(result.is_ok());
/// ```
pub fn insert_data(connection_string: &str, data: Vec<&str>) -> Result<(), Error> {
    let mut client = Client::connect(connection_string, NoTls)?;

    for row in data {
        client.batch_execute(&format!("INSERT INTO {}", row))?;
    }

    Ok(())
}

/// Create a new database from a CSV file.
///
/// # Arguments
///
/// * `connection_string` - A connection string to the PostgreSQL server.
/// * `db` - The name of the database to create.
/// * `file_path` - The path to the CSV file.
///
/// # Returns
///
/// A `Result` indicating whether the operation was successful.
///
/// # Example
///
/// ```ignore
/// use database::create_db_from_csv;
///
/// let connection_string = "postgresql://postgres:@localhost";
/// let db_name = "moma";
/// let file_path = "artist.csv";
/// let table_name = "artist";
///
/// let result = create_db_from_csv(connection_string, db_name, file_path, table_name);
/// assert!(result.is_ok());
/// ```
pub fn create_db_from_csv(
    connection_string: &str,
    db: &str,
    file_path: &str,
    table_name: &str,
) -> Result<(), Error> {
    create_db(connection_string, db)?;

    let file = File::open(file_path).unwrap();
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let headers = rdr.headers().unwrap().clone();

    let columns: Vec<String> = headers
        .iter()
        .map(|h| format!("{} VARCHAR NULL", h.replace(" ", "_")))
        .collect();

    let columns_clean: Vec<String> = headers.iter().map(|h| h.replace(" ", "_")).collect();

    let create_table_query = format!(
        "CREATE TABLE IF NOT EXISTS {} (id SERIAL PRIMARY KEY,{})",
        table_name,
        columns.join(", ")
    );

    let connection_string_full = format!("{}/{}", connection_string, db);
    let mut client = Client::connect(&connection_string_full, NoTls)?;
    client.batch_execute(&create_table_query)?;

    for result in rdr.records() {
        let record = result.unwrap();
        let values: Vec<String> = record
            .iter()
            .map(|v| format!("'{}'", v.replace("'", "''")))
            .collect();
        let insert_query = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name,
            columns_clean.join(", "),
            values.join(", ")
        );
        client.batch_execute(&insert_query)?;
    }

    Ok(())
}
