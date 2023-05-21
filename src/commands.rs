use indoc::indoc;
use regex::Regex;
use std::io::Result;
use std::io::{Error, ErrorKind};
use std::process::Command;

pub fn execute_command(command: &str) -> Result<String> {
    let output = Command::new("sh").arg("-c").arg(command).output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!(
            "Command `{}` failed with error `{}`.",
            command,
            stderr.to_string()
        );
        Err(Error::new(ErrorKind::Other, stderr.to_string()))
    }
}

// Tries to check if the user issued a known command.
// If true, then it will return the command output.
// Otherwise it will return the user input that was passed.
pub fn user_commands(input: &str) -> Result<String> {
    let regex_pattern = r"^!(\w+)\s*(.*)?";

    let help = indoc!(
        "
        Possible commands: 
        !tool [command to run] (e.g. !tool kubectl --help) 
        !help (shows this help message)
        "
    );

    let regex = Regex::new(regex_pattern).unwrap();
    for captured in regex.captures_iter(input) {
        if let Some(word) = captured.get(1) {
            match word.as_str() {
                "help" => {
                    println!("{}", help);
                    return Err(Error::new(ErrorKind::Other, "Help"));
                }
                "tool" => {
                    if let Some(command) = captured.get(2) {
                        if command.as_str().len() > 0 {
                            println!("Executing: {}", &command.as_str());
                            return execute_command(&command.as_str());
                        }
                    }
                    eprintln!("No command was passed, see !help");
                    return Err(Error::new(ErrorKind::NotFound, "No command"));
                }
                _ => {
                    eprintln!("Unknown command: {}", word.as_str());
                    println!("{}", help);
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "Unknown command: {}".to_owned() + word.as_str(),
                    ));
                }
            }
        }
    }
    Ok(input.to_owned())
}
