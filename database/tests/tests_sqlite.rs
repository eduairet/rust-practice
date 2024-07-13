use database::*;
use rusqlite::params;
use shared::Cat;

#[cfg(test)]
mod tests_sqlite {
    use super::*;

    #[test]
    #[ignore]
    fn test_create_sqlite_database() {
        let database = "test.db";
        let result = create_sqlite_cats_database(database);
        assert!(result.is_ok());
    }

    #[test]
    #[ignore]
    fn test_insert_select_cats() {
        let database = "test.db";

        delete_cats_database(database).unwrap();
        assert!(create_sqlite_cats_database(database).is_ok());

        let cats: Vec<Cat> = vec![
            Cat {
                name: String::from("Fluffy"),
                color: String::from("White"),
            },
            Cat {
                name: String::from("Whiskers"),
                color: String::from("Black"),
            },
            Cat {
                name: String::from("Socks"),
                color: String::from("Gray"),
            },
        ];

        let result = insert_select_cats(database, &cats);
        assert!(result.is_ok());

        delete_cats_database(database).unwrap();
        assert!(create_sqlite_cats_database(database).is_ok());
    }

    #[test]
    #[ignore]
    fn test_submit_db_transaction_commit() {
        let database = "test.db";

        delete_cats_database(database).unwrap();
        assert!(create_sqlite_cats_database(database).is_ok());

        let quey_no_params = "DELETE FROM cat_colors";
        let transaction = submit_db_transaction(
            database,
            &quey_no_params,
            params![],
            TransactionType::Commit,
        );

        assert!(transaction.is_ok());
    }

    #[test]
    #[ignore]
    fn test_submit_db_transaction_rollback() {
        let database = "test.db";

        delete_cats_database(database).unwrap();
        assert!(create_sqlite_cats_database(database).is_ok());

        let quey_no_params = "DELETE FROM cat_colors";
        let transaction = submit_db_transaction(
            database,
            &quey_no_params,
            params![],
            TransactionType::Rollback,
        );

        assert!(transaction.is_ok());
    }
}
