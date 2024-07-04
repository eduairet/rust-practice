use threads::*;

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
        let global_state = create_global_state();
        global_state_insert("BTC", global_state).unwrap();
        global_state_insert("ETH", global_state).unwrap();
        {
            let global_state = global_state.lock().unwrap();
            assert_eq!(global_state.len(), 2);
        }
    }
}
