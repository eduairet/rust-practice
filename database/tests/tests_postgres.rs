use database::{create_db, delete_db};

#[cfg(test)]
mod tests_postgres {
    use super::*;

    #[test]
    #[ignore]
    fn test_create_then_delete_db() {
        let db_name = "test_db";
        let connection_string = "postgresql://postgres:@localhost";

        let result = create_db(connection_string, db_name);
        assert!(result.is_ok());

        let result = delete_db(connection_string, db_name);
        assert!(result.is_ok());
    }
}
