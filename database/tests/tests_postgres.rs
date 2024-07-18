use database::{add_tables, create_db, create_db_from_csv, delete_db, insert_data};
use postgres::{Client, Error, NoTls};
use std::collections::HashMap;

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

    const TEST_TABLES: [&str; 2] = [
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
        let result = add_tables(&connection_string, Vec::from(TEST_TABLES));
        assert!(result.is_ok());
        let _ = fixture_delete_db(&connection_string, &db_name);
    }

    #[test]
    #[ignore]
    fn test_insert_and_query_data() {
        let (connection_string, db_name) = fixture_create_db();
        let result = add_tables(&connection_string, Vec::from(TEST_TABLES));
        assert!(result.is_ok());

        let mut authors = HashMap::new();
        authors.insert("Octavio Paz", "México");
        authors.insert("Jorge Luis Borges", "Argentina");

        let mut books = HashMap::new();
        books.insert("El laberinto de la soledad", "1");
        books.insert("Ficciones", "2");

        let mut data = Vec::new();

        for (author, country) in authors.iter() {
            data.push(format!(
                "author (name, country) VALUES ('{}', '{}')",
                author, country
            ));
        }

        for (title, author_id) in books.iter() {
            data.push(format!(
                "book (title, author_id) VALUES ('{}', {})",
                title, author_id
            ));
        }

        let result = insert_data(
            &connection_string,
            data.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
        );

        assert!(result.is_ok());

        let mut client = Client::connect(&connection_string, NoTls).unwrap();

        for row in client
            .query("SELECT name, country FROM author", &[])
            .unwrap()
        {
            let name: &str = row.get(0);
            let country: &str = row.get(1);
            assert!(authors.contains_key(name));
            assert_eq!(authors.get(name), Some(&country));
        }

        for row in client
            .query(
                "SELECT title, author.name FROM book 
                JOIN author ON book.author_id = author.id",
                &[],
            )
            .unwrap()
        {
            let title: &str = row.get(0);
            let author_name: &str = row.get(1);
            assert!(books.contains_key(title));
            assert!(authors.contains_key(author_name));
        }

        let _ = fixture_delete_db(&connection_string, &db_name);
    }

    #[test]
    #[ignore]
    fn test_create_db_from_csv() {
        let connection_string = "postgresql://postgres:@localhost";
        let db_name = "moma";
        let file_path = "artist.csv";
        let table_name = "artist";

        let _ = fixture_delete_db(&connection_string, &db_name);

        let result = create_db_from_csv(connection_string, db_name, file_path, table_name);
        assert!(result.is_ok());

        let connection_string_full = format!("{}/{}", connection_string, db_name);
        let mut client = Client::connect(&connection_string_full, NoTls).unwrap();

        for row in client
            .query(
                "SELECT Nationality, COUNT(Nationality) AS Count 
                FROM artist GROUP BY Nationality ORDER BY Count DESC",
                &[],
            )
            .unwrap()
        {
            let (nationality, count): (Option<String>, Option<i64>) = (row.get(0), row.get(1));

            assert!(nationality.is_some());
            assert!(count.is_some());
            println!("{:?} {:?}", nationality, count);
        }

        client.close().unwrap();
        let _ = fixture_delete_db(&connection_string, &db_name);
    }
}
