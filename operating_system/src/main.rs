use std::collections::HashSet;
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let mut child = Command::new("python3")
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    child
        .stdin
        .as_mut()
        .ok_or("Child process stdin has not been captured!")
        .unwrap()
        .write_all(b"import this; copyright(); credits(); exit()")
        .unwrap();

    let output = child.wait_with_output().unwrap();

    if output.status.success() {
        let raw_output = String::from_utf8(output.stdout).unwrap();
        let words = raw_output
            .split_whitespace()
            .map(|s| s.to_lowercase())
            .collect::<HashSet<_>>();
        println!("Found {} unique words:", words.len());
        println!("{:#?}", words);
    } else {
        let err = String::from_utf8(output.stderr).expect("External command failed:\n");
        eprintln!("{}", err);
    }
}
