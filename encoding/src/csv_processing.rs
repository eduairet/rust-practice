use csv::Error;

pub fn read_csv_records(csv_data: &str) -> Result<Vec<String>, Error> {
    let csv = csv_data;

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
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
