/// Colors enum
///
/// Example:
///
/// ```
/// use shared::Colors;
///
/// let color = Colors::Red;
/// assert_eq!(color, Colors::Red);
/// ```
#[derive(Debug, PartialEq)]
pub enum Colors {
    Red,
    Blue,
    Green,
}
