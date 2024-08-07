use csv::{Reader, Writer};
use encoding::{
    filter_csv_records_matching_predicate, read_csv_records, read_csv_records_custom_delimiter,
    MemeCoin, Row, Steak, Token,
};
use std::io::stdout;

#[cfg(test)]
mod tests_csv_processing {
    use super::*;

    #[test]
    fn test_read_csv_records() {
        let csv_data = "name,age\nAlice,30\nBob,25\n";
        let records = read_csv_records(csv_data).unwrap();
        print!("{:?}", records);
        assert_eq!(records, vec!["Alice30", "Bob25"]);
    }

    #[test]
    fn test_read_csv_records_strong_type() {
        let csv_data =
            "chain,name,ticker,price\nEthereum,ETH,Ether,3000.0\nBitcoin,BTC,Bitcoin,60000.0\n";
        let mut reader = csv::Reader::from_reader(csv_data.as_bytes());

        let records = reader
            .deserialize()
            .collect::<Result<Vec<Token>, csv::Error>>()
            .unwrap();

        let ether = &records[0];
        print!("{:?}", ether);
        assert_eq!(ether.chain, "Ethereum");
        assert_eq!(ether.name, "ETH");
        assert_eq!(ether.ticker, "Ether");
        assert_eq!(ether.price, 3000.0);

        let bitcoin = &records[1];
        print!("{:?}", bitcoin);
        assert_eq!(bitcoin.chain, "Bitcoin");
        assert_eq!(bitcoin.name, "BTC");
        assert_eq!(bitcoin.ticker, "Bitcoin");
        assert_eq!(bitcoin.price, 60000.0);
    }

    #[test]
    fn test_read_csv_records_custom_delimiter() {
        let csv_data = "name;age\nAlice;30\nBob;25\n";
        let records = read_csv_records_custom_delimiter(csv_data, b';').unwrap();
        print!("{:?}", records);
        assert_eq!(records, vec!["Alice30", "Bob25"]);
    }

    #[test]
    fn test_filter_csv_records_matching_predicate() {
        let csv_data = "name,age\nAlice,30\nBob,25\n";
        let query = "Alice";
        let filtered_records = filter_csv_records_matching_predicate(csv_data, query);
        print!("{:?}", filtered_records);
        assert_eq!(filtered_records, vec!["Alice30"]);
    }

    #[test]
    fn test_handle_invalid_csv_data() {
        let csv_data = "name,price,id\nT-bone,20.0,1\nPicanha,25.0";

        let mut reader = csv::Reader::from_reader(csv_data.as_bytes());
        let result = reader
            .deserialize::<Steak>()
            .map(|result| match result {
                Ok(steak) => steak,
                Err(err) => {
                    print!("{:?}", err);
                    Steak {
                        name: "Unknown".to_string(),
                        price: 0.0,
                        id: Option::None,
                    }
                }
            })
            .collect::<Vec<Steak>>();

        print!("{:?}", result);

        assert_eq!(result[0].name, "T-bone");
        assert_eq!(result[0].price, 20.0);
        assert_eq!(result[0].id, Some(1));

        assert_eq!(result[1].name, "Unknown");
        assert_eq!(result[1].price, 0.0);
        assert_eq!(result[1].id, Option::None);
    }

    #[test]
    fn test_serialize_records_to_csv() {
        let mut writter = csv::Writer::from_writer(stdout());
        writter.write_record(&["name", "age"]).unwrap();

        writter.serialize(("Alice", 30)).unwrap();
        writter.serialize(("Bob", 25)).unwrap();

        writter.flush().unwrap();

        assert!(true);
    }

    #[test]
    fn test_serialize_records_to_csv_serde() {
        let mut writter = csv::Writer::from_writer(stdout());

        let pepe = MemeCoin {
            chain: "Ethereum",
            name: "Pepe",
            ticker: "PEPE",
        };

        let wif = MemeCoin {
            chain: "Solana",
            name: "Dog Wif Hat",
            ticker: "WIF",
        };

        writter.serialize(pepe).unwrap();
        writter.serialize(wif).unwrap();

        writter.flush().unwrap();

        assert!(true);
    }

    #[test]
    fn test_transform_csv_column() {
        let csv_data =
            "color_name,color\nRed,\"255,0,0\"\nGreen,\"0,255,0\"\nBlue,\"0,0,255\"\n".to_owned();
        let mut out = Writer::from_writer(vec![]);
        let mut reader = Reader::from_reader(csv_data.as_bytes());
        for result in reader.deserialize::<Row>() {
            let res = result.unwrap();
            out.serialize((
                res.color_name,
                res.color.red,
                res.color.green,
                res.color.blue,
            ))
            .unwrap();
        }
        let written = String::from_utf8(out.into_inner().unwrap()).unwrap();
        print!("{:?}", written);
        assert_eq!(Some("Blue,0,0,255"), written.lines().last(),);
    }
}
