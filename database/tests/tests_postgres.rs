use database::{add_tables, create_db, delete_db};
use postgres::Error;

#[cfg(test)]
mod tests_postgres {

    use super::*;

    fn fixture_set_db() -> (String, String) {
        (
            String::from("postgresql://postgres:@localhost"),
            String::from("test_db"),
        )
    }

    fn fixture_create_db() -> (String, String) {
        let (connection_string, db_name) = fixture_set_db();
        let result = create_db(&connection_string, &db_name);
        assert!(result.is_ok());
        (connection_string, db_name)
    }

    fn fixture_delete_db(connection_string: &str, db_name: &str) -> Result<(), Error> {
        let result = delete_db(connection_string, db_name);
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_create_then_delete_db() {
        let (connection_string, db_name) = fixture_create_db();
        let _ = fixture_delete_db(&connection_string, &db_name);
    }

    #[test]
    #[ignore]
    fn test_add_tables() {
        let (connection_string, db_name) = fixture_create_db();
        let tables: Vec<&str> = vec![
            "author (
                id              SERIAL PRIMARY KEY,
                name            VARCHAR NOT NULL,
                country         VARCHAR NOT NULL
            )",
            "book  (
                id              SERIAL PRIMARY KEY,
                title           VARCHAR NOT NULL,
                author_id       INTEGER NOT NULL REFERENCES author
            )",
        ];
        let result = add_tables(&connection_string, tables);
        assert!(result.is_ok());
        let _ = fixture_delete_db(&connection_string, &db_name);
    }
}
