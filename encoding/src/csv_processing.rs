use csv::{Error, ReaderBuilder};
use serde::Deserialize;

/// Read CSV records from a string.
///
/// # Arguments
///
/// * `csv_data` - A string slice that holds the CSV data.
///
/// # Returns
///
/// A vector of strings, where each string is a record from the CSV data.
///
/// # Example
///
/// ```
/// use encoding::read_csv_records;
///
/// let csv_data = "name,age\nAlice,30\nBob,25\n";
///
/// let records = read_csv_records(csv_data).unwrap();
///
/// assert_eq!(records, vec!["Alice30", "Bob25"]);
/// ```
pub fn read_csv_records(csv_data: &str) -> Result<Vec<String>, Error> {
    let mut reader = csv::Reader::from_reader(csv_data.as_bytes());
    let mut records = Vec::new();

    for result in reader.records() {
        let record = result?;
        records.push(
            record
                .iter()
                .map(|field| field.to_string())
                .collect::<String>(),
        );
    }

    Ok(records)
}

/// A token struct that represents a record in a CSV file.
///
/// # Example
///
/// ```
/// use encoding::Token;
///
/// let token = Token {
///    chain: "Ethereum".to_string(),
///    name: "ETH".to_string(),
///    ticker: "Ether".to_string(),
///    price: 3000.0,
/// };
///
/// assert_eq!(token.chain, "Ethereum");
/// ```
#[derive(Debug, Deserialize)]
pub struct Token {
    pub chain: String,
    pub name: String,
    pub ticker: String,
    pub price: f64,
}

/// Read CSV records from a string and deserialize them into a strong type.
///
/// # Arguments
///
/// * `csv_data` - A string slice that holds the CSV data.
/// * `delimiter` - A u8 that represents the delimiter character.
///
/// # Returns
///
/// A vector of tokens, where each token is a record from the CSV data.
///
/// # Example
///
/// ```
/// use encoding::{read_csv_records_custom_delimiter, Token};
///
/// let csv_data = "name;age\nAlice;30\nBob;25\n";
/// let records = read_csv_records_custom_delimiter(csv_data, b';').unwrap();
///
/// assert_eq!(records, vec!["Alice30", "Bob25"]);
/// ```
pub fn read_csv_records_custom_delimiter(
    csv_data: &str,
    delimiter: u8,
) -> Result<Vec<String>, Error> {
    let mut reader = ReaderBuilder::new()
        .delimiter(delimiter)
        .from_reader(csv_data.as_bytes());
    let mut records = Vec::new();

    for result in reader.records() {
        let record = result?;
        records.push(
            record
                .iter()
                .map(|field| field.to_string())
                .collect::<String>(),
        );
    }

    Ok(records)
}
