use threads::*;

#[cfg(test)]
mod tests_explicit_threads {

    use super::*;

    #[test]
    fn test_mutate_array_in_parallel() {
        let mut array = [1, 2, 3, 4, 5];
        let result = mutate_array_in_parallel(&mut array, 1);
        assert_eq!(result, [2, 3, 4, 5, 6]);
    }
}
