use crate::Colors;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use serde::Deserialize;
use std::fmt;

/// Point struct
///
/// Example:a
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

/// Commit struct
///
/// Example:
///
/// ```
/// use shared::Commit;
///
/// let commit = Commit {
///    hash: "1234567890abcdef".to_string(),
///    message: "Initial commit".to_string(),
/// };
/// assert_eq!(commit.hash, "1234567890abcdef");
/// assert_eq!(commit.message, "Initial commit");
/// ```
#[derive(PartialEq, Default, Clone, Debug)]
pub struct Commit {
    pub hash: String,
    pub message: String,
}

/// PhoneNumber struct
///
/// Example:
///
/// ```
/// use shared::PhoneNumber;
///
/// let phone_number = PhoneNumber {
///    area: "415",
///    exchange: "555",
///    subscriber: "1234",
/// };
///
/// assert_eq!(phone_number.to_string(), "(415) 555-1234");
/// ```
pub struct PhoneNumber<'a> {
    pub area: &'a str,
    pub exchange: &'a str,
    pub subscriber: &'a str,
}
impl<'a> fmt::Display for PhoneNumber<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) {}-{}", self.area, self.exchange, self.subscriber)
    }
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub login: String,
    pub id: u32,
}
