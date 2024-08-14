use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_file_lines(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).unwrap();
    let file_buffered = BufReader::new(file);
    file_buffered.lines().map(|line| line.unwrap()).collect()
}
