use operating_system::run_external_command_process_stdout;
use regex::Regex;
use shared::Commit;

#[cfg(test)]
mod tests_external_command {
    use super::*;

    #[test]
    fn test_run_external_command_process_stdout() {
        let command = "git";
        let args = vec!["log", "--oneline"];
        let output = run_external_command_process_stdout(command, args).unwrap();

        let pattern = Regex::new(
            r"(?x)
        ([0-9a-fA-F]+) # commit hash
        (.*)           # The commit message",
        )
        .unwrap();

        let result: Vec<Commit> = output
            .lines()
            .filter_map(|line| pattern.captures(line))
            .map(|cap| Commit {
                hash: cap[1].to_string(),
                message: cap[2].trim().to_string(),
            })
            .take(5)
            .collect();

        print!("{:?}", result);

        let hash_pattern = Regex::new(r"^[0-9a-fA-F]{7}$").unwrap();
        assert_eq!(result.len(), 5);
        assert!(hash_pattern.is_match(&result[0].hash));
    }
}
