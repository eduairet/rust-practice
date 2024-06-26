use command_line::create_cmd;

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
        let num: &String = m.get_one("num").expect("required");
        let guess: i32 = num.parse::<i32>().unwrap() + 5;
        assert_eq!(guess, 8);
    }
}
