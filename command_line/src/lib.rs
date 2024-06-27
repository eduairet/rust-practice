use ansi_term::{Colour, Style};
use clap::{Arg, ArgAction, Command};
use shared::{Colors, TerminalColor};

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
                .action(ArgAction::Set)
                .help("Five less than your favorite number"),
        );
    app
}

/// Print a message to the command line with custom formatting
///
/// # Arguments
///
/// * `message` - A string slice that holds the message to print
///
/// * `colors` - A vector of `TerminalColor` instances that hold the color and boldness of the message
///
/// # Examples
///
/// ```
/// use shared::{Colors, TerminalColor};
/// use command_line::formatted_cli_message;
///
/// let message = "Hello, world!";
/// let colors = vec![
///   TerminalColor::new(Some(Colors::Red), false),
///   TerminalColor::new(Some(Colors::Green), true),
///   TerminalColor::new(Some(Colors::Blue), false),
///   TerminalColor::new(None, true),
///   TerminalColor::new(None, false),
/// ];
/// formatted_cli_message(message, colors);
/// ```
pub fn formatted_cli_message(message: &str, colors: Vec<TerminalColor>) {
    for color in colors {
        let color_str = match color.value {
            Some(Colors::Red) => Colour::Red.normal(),
            Some(Colors::Green) => Colour::Green.normal(),
            Some(Colors::Blue) => Colour::Blue.normal(),
            None => Style::new(),
        };

        let color_str = if color.bold {
            color_str.bold().paint(message)
        } else {
            color_str.paint(message)
        };

        println!("{}", color_str);
    }
}
