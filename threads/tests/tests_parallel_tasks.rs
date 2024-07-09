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

    #[test]
    fn test_match_predicate_in_parallel_all() {
        let array = [1, 2, 3, 4, 5];

        let is_all = true;
        let predicate = |x| x > 0;
        let result = match_predicate_in_parallel(&array, is_all, predicate);
        assert_eq!(result, true);

        let is_all = true;
        let predicate = |x| x > 5;
        let result = match_predicate_in_parallel(&array, is_all, predicate);
        assert_eq!(result, false);
    }

    #[test]
    fn test_match_predicate_in_parallel_any() {
        let array = [1, 2, 3, 4, 5];

        let is_all = false;
        let predicate = |x| x > 5;
        let result = match_predicate_in_parallel(&array, is_all, predicate);
        assert_eq!(result, false);

        let is_all = false;
        let predicate = |x| x > 4;
        let result = match_predicate_in_parallel(&array, is_all, predicate);
        assert_eq!(result, true);
    }

    #[test]
    fn test_match_predicate_in_parallel_all_any_even() {
        let predicate = |x| x % 2 == 0;
        let array = [1, 2, 3, 4, 5];

        let is_all = true;
        let result = match_predicate_in_parallel(&array, is_all, predicate);
        assert_eq!(result, false);

        let is_all = false;
        let result = match_predicate_in_parallel(&array, is_all, predicate);
        assert_eq!(result, true);

        let array = [2, 4, 6, 8, 10];

        let is_all = true;
        let result = match_predicate_in_parallel(&array, is_all, predicate);
        assert_eq!(result, true);
    }
}
