use csv::{Error, ReaderBuilder};
use serde::{Deserialize, Serialize};

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

/// Filter CSV records that match a predicate.
///
/// # Arguments
///
/// * `csv_data` - A string slice that holds the CSV data.
/// * `query` - A string slice that holds the query to filter the records.
///
/// # Returns
///
/// A vector of strings, where each string is a record from the CSV data that matches the query.
///
/// # Example
///
/// ```
/// use encoding::filter_csv_records_matching_predicate;
///
/// let csv_data = "name,age\nAlice,30\nBob,25\n";
/// let query = "Alice";
///
/// let filtered_records = filter_csv_records_matching_predicate(csv_data, query);
///
/// assert_eq!(filtered_records, vec!["Alice30"]);
/// ```
pub fn filter_csv_records_matching_predicate(csv_data: &str, query: &str) -> Vec<String> {
    let mut reader = ReaderBuilder::new().from_reader(csv_data.as_bytes());
    let mut filtered_records: Vec<String> = Vec::new();

    for result in reader.records() {
        let record = result.unwrap();
        let record_str = record
            .iter()
            .map(|field| field.to_string())
            .collect::<String>();

        if record_str.contains(query) {
            filtered_records.push(record_str);
        }
    }

    filtered_records
}

/// A struct that represents a steak record in a CSV file.
///
/// # Example
///
/// ```
/// use encoding::Steak;
///
/// let steak = Steak {
///    name: "T-bone".to_string(),
///    price: 20.0,
///    id: Some(1),
/// };
///
/// assert_eq!(steak.name, "T-bone");
/// ```
#[derive(Debug, Deserialize)]
pub struct Steak {
    pub name: String,
    pub price: f64,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub id: Option<u64>,
}

/// Serializable struct that represents a meme coin.
///
/// # Example
///
/// ```
/// use encoding::MemeCoin;
///
/// let meme_coin = MemeCoin {
///    chain: "Ethereum",
///    name: "Pepe",
///    ticker: "PEPE"
/// };
///
/// assert_eq!(meme_coin.chain, "Ethereum");
/// ```
#[derive(Debug, Serialize)]
pub struct MemeCoin<'a> {
    pub chain: &'a str,
    pub name: &'a str,
    pub ticker: &'a str,
}
