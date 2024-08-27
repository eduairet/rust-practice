use std::{
    collections::HashSet,
    env, fs,
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind, Write},
    process::{Command, Stdio},
};

fn main() {
    // Run an external command passing it stdin and check for an error code

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

    // Run an external command passing it stdin and check for an error code

    let directory = std::env::current_dir().unwrap();
    let mut du_output_child = Command::new("du")
        .arg("-ah")
        .arg(&directory)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(du_output) = du_output_child.stdout.take() {
        let mut sort_output_child = Command::new("sort")
            .arg("-hr")
            .stdin(du_output)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        du_output_child.wait().unwrap();

        if let Some(sort_output) = sort_output_child.stdout.take() {
            let head_output_child = Command::new("head")
                .args(&["-n", "10"])
                .stdin(sort_output)
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

            let head_stdout = head_output_child.wait_with_output().unwrap();

            sort_output_child.wait().unwrap();

            println!(
                "Top 10 biggest files and directories in '{}':\n{}",
                directory.display(),
                String::from_utf8(head_stdout.stdout).unwrap()
            );
        }
    }

    // Redirect both stdout and stderr of child process to the same file

    let outputs = File::create("out.txt").unwrap();
    let errors = outputs.try_clone().unwrap();

    Command::new("ls")
        .args(&[".", "oops"])
        .stdout(Stdio::from(outputs))
        .stderr(Stdio::from(errors))
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();

    // Continuously process child process' outputs

    let stdout = Command::new("grep")
        .args(&["-r", "usb", "."])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))
        .unwrap();

    let reader = BufReader::new(stdout);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| line.find("usb").is_some())
        .for_each(|line| println!("{}", line));

    // Read Environment Variable

    let config_path = env::var("CONFIG").unwrap_or("./config/.env".to_string());

    let config: String = fs::read_to_string(config_path).unwrap();
    println!("Config: {}", config);
}
