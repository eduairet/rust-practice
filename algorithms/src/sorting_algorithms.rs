use shared::Person;
use std::cmp::Ordering;

/// Sorts a vector of numbers in ascending order.
///
/// # Arguments
///
/// * `vec` - A mutable reference to a vector of numbers.
///
/// # Example
///
/// ```
/// use algorithms::sort_num_vector;
///
/// let mut vec = vec![3, 1, 2];
/// sort_num_vector(&mut vec);
/// println!("{:?}", vec); // [1, 2, 3]
/// ```
pub fn sort_num_vector<T>(vec: &mut Vec<T>)
where
    T: PartialOrd,
{
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less));
}

/// Sorts a vector of people by name or age.
///
/// # Arguments
///
/// * `vec` - A mutable reference to a vector of people.
/// * `by_name` - A boolean that indicates if the vector should be sorted by name.
/// * `by_age` - A boolean that indicates if the vector should be sorted by age.
///
/// # Example
///
/// ```
/// use algorithms::sort_people;
/// use shared::Person;
///
/// let mut people = vec![
///   Person::new("John".to_string(), 25),
///   Person::new("Jane".to_string(), 20),
///   Person::new("Alice".to_string(), 30),
/// ];
///
/// sort_people(&mut people, true, false);
/// println!("{:?}", people); // [Person { name: "Alice", age: 30 }, Person { name: "Jane", age: 20 }, Person { name: "John", age: 25 }]
/// ```
pub fn sort_people(vec: &mut Vec<Person>, by_name: bool, by_age: bool) {
    vec.sort_by(|a, b| {
        if by_name && !by_age {
            a.name.partial_cmp(&b.name).unwrap_or(Ordering::Less)
        } else if !by_name && by_age {
            a.age.partial_cmp(&b.age).unwrap_or(Ordering::Less)
        } else {
            Ordering::Less
        }
    });
}
