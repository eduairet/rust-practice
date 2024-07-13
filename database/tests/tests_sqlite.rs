use database::*;

#[cfg(test)]
mod tests_sqlite {
    use super::*;

    #[test]
    #[ignore]
    fn test_create_sqlite_database() {
        let database = "test.db";
        let result = create_sqlite_database(database);
        assert!(result.is_ok());
    }
}
