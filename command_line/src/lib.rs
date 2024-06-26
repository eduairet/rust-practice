use clap::{Arg, ArgAction, Command};

/// Create a command line application
///
/// # Returns
///
/// A `Command` instance
///
/// # Examples
///
/// ```
/// use command_line::create_cmd;
///
/// let mut cmd = create_cmd();
/// let m = cmd
///    .try_get_matches_from_mut(["cmd", "--file", "input.txt", "--number", "3"])
///   .unwrap();
/// let file: &String = m.get_one("file").expect("default");
/// let num: &String = m.get_one("num").expect("required");
/// println!("File: {}, Number: {}", file, num); // File: input.txt, Number: 3
/// ```
pub fn create_cmd() -> Command {
    let app = Command::new("cmd")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .action(ArgAction::Set)
                .default_value("input.txt")
                .help("The file to work with. Defaults to input.txt"),
        )
        .arg(
            Arg::new("num")
                .short('n')
                .long("number")
                .required(true)
                .action(ArgAction::Set)
                .help("Five less than your favorite number"),
        );
    app
}
