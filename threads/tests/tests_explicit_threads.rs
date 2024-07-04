use lazy_static::lazy_static;
use std::{error::Error, sync::Mutex};
use threads::{find_max, parallel_pipeline, pass_data_between_two_threads};

#[cfg(test)]
mod tests_explicit_threads {
    use super::*;

    #[test]
    fn test_find_max() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let threshold = 5;
        assert_eq!(Some(10), find_max(&arr, threshold));
    }

    #[test]
    fn test_parallel_pipeline() {
        parallel_pipeline(10, 2);
    }

    #[test]
    fn test_pass_data_between_two_threads() {
        let num_messages = 3;
        let (.., receiver) = pass_data_between_two_threads(num_messages);
        for i in 0..num_messages {
            assert_eq!(i, receiver.recv().unwrap());
        }
    }

    #[test]
    fn test_maintain_global_state() {
        lazy_static! {
            static ref CRYPTOS: Mutex<Vec<String>> = Mutex::new(Vec::new());
        }

        fn insert(token: &str) -> Result<(), Box<dyn Error>> {
            let mut db = CRYPTOS.lock().map_err(|_| "Failed to acquire MutexGuard")?;
            db.push(token.to_string());
            Ok(())
        }

        insert("BTC").unwrap();
        insert("ETH").unwrap();

        let db = CRYPTOS.lock().unwrap();
        assert_eq!(db.len(), 2);
    }
}
