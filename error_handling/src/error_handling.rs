use serde::{Deserialize, Deserializer};
use std::{error::Error, fs::File, io::Read};

/// Read the uptime from a file
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the file
///
/// # Returns
///
/// * `Result<usize, Box<dyn Error>>` - A Result that holds the uptime in seconds or an error
///
/// # Example
///
/// ```
/// use error_handling::read_uptime;
///
/// let uptime = read_uptime("Cargo.toml");
/// assert!(uptime.is_ok());
/// ```
pub fn read_uptime(file_path: &str) -> Result<usize, Box<dyn Error>> {
    let mut uptime = String::new();

    let file = File::open(file_path)?.read_to_string(&mut uptime);

    let file_content = match file {
        Ok(content) => content,
        Err(e) => return Err(Box::new(e)),
    };

    Ok(file_content)
}

/// A struct that represents an RGB color
///
/// # Example
///
/// ```
/// use error_handling::Rgb;
///
/// let color = Rgb {
///    red: 255,
///    green: 0,
///    blue: 0,
/// };
///
/// assert_eq!(color.red, 255);
/// ```
#[derive(Debug)]
pub struct Rgb {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
}

impl<'de> Deserialize<'de> for Rgb {
    /// Deserialize an RGB color
    ///
    /// # Arguments
    ///
    /// * `deserializer` - A Deserializer that holds the RGB color
    ///
    /// # Returns
    ///
    /// * `Result<Self, D::Error>` - A Result that holds the RGB color or an error
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let color: Vec<u8> = Deserialize::deserialize(deserializer)?;
        match color.len() {
            3 => Ok(Rgb {
                red: color[0],
                green: color[1],
                blue: color[2],
            }),
            _ => Err(serde::de::Error::custom("Invalid RGB color")),
        }
    }
}

impl Rgb {
    /// Create an RGB color from a CSV reader
    ///
    /// # Arguments
    ///
    /// * `csv_data` - A slice of bytes that holds the CSV data
    ///
    /// # Returns
    ///
    /// * `Result<Rgb, Box<dyn Error>>` - A Result that holds the RGB color or an error
    ///
    /// # Example
    ///
    /// ```
    /// use error_handling::Rgb;
    ///
    /// let csv_data = b"red,blue,green\n255,0,0\n";
    /// let rgb = Rgb::from_reader(csv_data).unwrap();
    /// assert_eq!(rgb.red, 255);
    /// ```
    pub fn from_reader(csv_data: &[u8]) -> Result<Rgb, Box<dyn Error>> {
        let color: Rgb = csv::Reader::from_reader(csv_data)
            .deserialize()
            .nth(0)
            .ok_or_else(|| "Cannot deserialize the first CSV record".to_string())?
            .map_err(|e| format!("Cannot deserialize RGB color: {}", e))?;

        Ok(color)
    }
}
