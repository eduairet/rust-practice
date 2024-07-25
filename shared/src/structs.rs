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
/// assert_eq!(point.x, 1);
/// assert_eq!(point.y, 2);
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
    /// assert!(point.x >= std::i32::MIN);
    /// assert!(point.x <= std::i32::MAX);
    /// assert!(point.y >= std::i32::MIN);
    /// assert!(point.y <= std::i32::MAX);
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
/// assert_eq!(person.name, "John");
/// assert_eq!(person.age, 30);
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
/// assert_eq!(terminal_color.value.unwrap(), Colors::Red);
/// assert!(terminal_color.bold);
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

/// Cat struct
///
/// Example:
///
/// ```
/// use shared::Cat;
///
/// let cat = Cat { name: "Whiskers".to_string(), color: "Black".to_string() };
/// assert_eq!(cat.name, "Whiskers");
/// assert_eq!(cat.color, "Black");
/// ```
#[derive(Debug)]
pub struct Cat {
    pub name: String,
    pub color: String,
}
