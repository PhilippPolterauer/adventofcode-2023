use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_cli_command() {
    let mut cmd = Command::cargo_bin("adventofcode").unwrap();

    for day in 1..25 {
        for part in 0..2 {
            // Add command-line arguments and options as needed
            cmd.arg(day.to_string()).arg(part.to_string());

            let assert = cmd.assert();

            // Use assert_cmd's methods to check the output, exit code, etc.
            assert.success();
        }
    }
}
