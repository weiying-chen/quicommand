use std::{io::Write, process::Command};

pub struct CmdRunner {
    pub command: std::process::Command,
}

impl CmdRunner {
    pub fn new(command_string: &str, input: Option<&str>) -> CmdRunner {
        let mut command = Command::new("script");

        if let Some(input_str) = input {
            command
                .arg("-qec")
                .arg(command_string.replace("{}", input_str));
        } else {
            command.arg("-qec").arg(command_string);
        }

        command.arg("/dev/null");

        CmdRunner { command }
    }

    pub fn execute(&mut self, stdout: &mut impl Write) {
        let output = self.command.output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    let stdout_str = String::from_utf8_lossy(&output.stdout);

                    for line in stdout_str.lines() {
                        write!(stdout, "{}\r\n", line).unwrap();
                    }
                } else {
                    let stdout_str = String::from_utf8_lossy(&output.stdout);
                    let stderr_str = String::from_utf8_lossy(&output.stderr);

                    // To-do: see if this can be removed.
                    // This places the output on a new line.
                    write!(stdout, "\r\n").unwrap();
                    write!(stdout, "Exit status: {}\r\n", output.status).unwrap();
                    write!(stdout, "Standard error: {}\r\n", stderr_str.trim()).unwrap();
                    write!(stdout, "Standard output: {}\r\n", stdout_str.trim()).unwrap();
                }
            }
            Err(e) => {
                write!(stdout, "Error executing command: {:?}\r\n", e).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_ok() {
        let mut cmd_runner = CmdRunner::new("echo {}", Some("hello"));
        let mut stdout = Vec::new();

        cmd_runner.execute(&mut stdout);

        let stdout_str = String::from_utf8(stdout).unwrap();

        assert_eq!(stdout_str.trim(), "hello");
    }

    #[test]
    fn test_execute_status() {
        let mut cmd_runner = CmdRunner::new("exit 1", Some(""));
        let mut stdout = Vec::new();

        cmd_runner.execute(&mut stdout);

        let stderr_str = String::from_utf8_lossy(&stdout);

        assert!(stderr_str.contains("exit status: 1"));
    }

    #[test]
    fn test_execute_err() {
        let mut cmd_runner = CmdRunner::new("non-existent-command", Some(""));
        let mut stdout = Vec::new();

        cmd_runner.execute(&mut stdout);

        let stderr_str = String::from_utf8_lossy(&stdout);

        assert!(stderr_str.contains("command not found"));
    }
}
