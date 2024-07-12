use crate::Colors;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

/// Point struct
///
/// Example:
///
/// ```
/// use shared::Point;
///
/// let point = Point { x: 1, y: 2 };
/// println!("{:?}", point); // Point { x: 1, y: 2 }
/// ```
#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Distribution<Point> for Standard {
    /// Generate a random Point
    ///
    /// Example:
    ///
    /// ```
    /// use rand::Rng;
    /// use shared::Point;
    ///
    /// let mut rng = rand::thread_rng();
    /// let point: Point = rng.gen();
    /// println!("{:?}", point); // Point { x: 1, y: 2 }
    /// ```
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

/// Person struct
///
/// Example:
///
/// ```
/// use shared::Person;
///
/// let person = Person::new("John".to_string(), 30);
/// println!("{:?}", person); // Person { name: "John", age: 30 }
/// ```
#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u32,
}
impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

/// TerminalColor struct
///
/// Example:
///
/// ```
/// use shared::{TerminalColor, Colors};
///
/// let terminal_color = TerminalColor::new(Some(Colors::Red), true);
/// println!("{:?}", terminal_color); // TerminalColor { value: Some(Red), bold: true }
/// ```
#[derive(Debug)]
pub struct TerminalColor {
    pub value: Option<Colors>,
    pub bold: bool,
}
impl TerminalColor {
    pub fn new(value: Option<Colors>, bold: bool) -> Self {
        TerminalColor { value, bold }
    }
}
