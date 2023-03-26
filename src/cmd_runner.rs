use std::{io::Write, process::Command};

pub struct CmdRunner {
    pub command: std::process::Command,
}

impl CmdRunner {
    pub fn new(command_string: &str, input_placeholder: &str) -> CmdRunner {
        let mut command = Command::new("script");

        command
            .arg("-qec")
            .arg(command_string.replace("{}", input_placeholder))
            .arg("/dev/null");

        println!("command: {:?}", command);
        CmdRunner { command }
    }

    pub fn execute(&mut self, stdout: &mut impl Write) {
        let output = self.command.output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    let stdout_str = String::from_utf8_lossy(&output.stdout);

                    // This places the output on a new line.
                    write!(stdout, "\r\n").unwrap();

                    for line in stdout_str.lines() {
                        write!(stdout, "{}\r\n", line).unwrap();
                    }
                } else {
                    let stdout_str = String::from_utf8_lossy(&output.stdout);
                    let stderr_str = String::from_utf8_lossy(&output.stderr);

                    // This places the output on a new line.
                    write!(stdout, "\r\n").unwrap();

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
