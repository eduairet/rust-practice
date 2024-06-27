use command_line::{create_cmd, formatted_cli_message};
use shared::{Colors, TerminalColor};

#[cfg(test)]
mod tests_command_line {
    use super::*;

    #[test]
    fn test_create_cmd() {
        let mut cmd = create_cmd();
        let m = cmd
            .try_get_matches_from_mut(["cmd", "--file", "input.txt", "--number", "3"])
            .unwrap();
        let file: &String = m.get_one("file").expect("default");
        assert_eq!(file, "input.txt");
        let num: &String = m.get_one("num").expect("default");
        let guess: i32 = num.parse::<i32>().unwrap() + 5;
        assert_eq!(guess, 8);
    }

    #[test]
    fn test_formatted_cli_message() {
        let message = "Hello, world!";
        let colors = vec![
            TerminalColor::new(Some(Colors::Red), false),
            TerminalColor::new(Some(Colors::Green), true),
            TerminalColor::new(Some(Colors::Blue), false),
            TerminalColor::new(None, true),
            TerminalColor::new(None, false),
        ];
        formatted_cli_message(message, colors);
    }
}
