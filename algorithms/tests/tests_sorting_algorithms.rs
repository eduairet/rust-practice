use algorithms::{sort_num_vector, sort_people};
use shared::Person;

#[cfg(test)]
mod tests_sorting_algorithms {

    use super::*;

    #[test]
    fn test_sort_num_vector_i32() {
        let mut vec = vec![3, 1, 2];
        sort_num_vector(&mut vec);
        assert_eq!(vec, [1, 2, 3]);
    }

    #[test]
    fn test_sort_num_vector_f64() {
        let mut vec = vec![3.0, 1.0, 2.0];
        sort_num_vector(&mut vec);
        assert_eq!(vec, [1.0, 2.0, 3.0]);
    }

    fn create_people_fixture() -> Vec<Person> {
        vec![
            Person::new("John".to_string(), 25),
            Person::new("Jane".to_string(), 20),
            Person::new("Alice".to_string(), 30),
        ]
    }

    #[test]
    fn test_sort_people_by_name() {
        let mut people = create_people_fixture();

        sort_people(&mut people, true, false);
        assert_eq!(
            format!("{:?}", people),
            format!(
                "{:?}",
                vec![
                    Person::new("Alice".to_string(), 30),
                    Person::new("Jane".to_string(), 20),
                    Person::new("John".to_string(), 25),
                ]
            )
        );
    }

    #[test]
    fn test_sort_people_by_age() {
        let mut people = create_people_fixture();

        sort_people(&mut people, false, true);
        assert_eq!(
            format!("{:?}", people),
            format!(
                "{:?}",
                vec![
                    Person::new("Jane".to_string(), 20),
                    Person::new("John".to_string(), 25),
                    Person::new("Alice".to_string(), 30),
                ]
            )
        );
    }

    #[test]
    fn test_sort_people_by_name_and_age() {
        let mut people = create_people_fixture();

        sort_people(&mut people, true, true);
        assert_eq!(
            format!("{:?}", people),
            format!(
                "{:?}",
                vec![
                    Person::new("Alice".to_string(), 30),
                    Person::new("Jane".to_string(), 20),
                    Person::new("John".to_string(), 25),
                ]
            )
        );
    }
}
