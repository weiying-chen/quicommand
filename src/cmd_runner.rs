use std::{
    io::{BufRead, BufReader},
    process::{Command, Output, Stdio},
};

use crate::utils::escape_backticks;

pub enum CmdType {
    Interactive,
    Script,
}

pub struct CmdRunner {
    pub command: std::process::Command,
    pub command_type: CmdType,
}

impl CmdRunner {
    pub fn new(command_string: String, input: Option<String>) -> CmdRunner {
        let mut command = Command::new("script");

        command.arg("-qec");

        if let Some(input_str) = input {
            let input_str = escape_backticks(&input_str);

            command.arg(command_string.replace("{}", &input_str));
        } else {
            command.arg(command_string.clone());
        }

        command.arg("/dev/null");

        let interactive_commands = ["hx", "vi"];

        let is_interactive_command = interactive_commands
            .iter()
            .any(|cmd| command_string.contains(cmd));

        let command_type = if is_interactive_command {
            CmdType::Interactive
        } else {
            CmdType::Script
        };

        CmdRunner {
            command,
            command_type,
        }
    }

    pub fn run(&mut self) -> Result<Output, std::io::Error> {
        let child = self.command.spawn().expect("failed to spawn command");
        let output = child.wait_with_output()?;

        Ok(output)
    }

    pub fn run_with_output(&mut self) -> Result<Output, std::io::Error> {
        // This prevents the output from becoming messed up in tests.
        self.command.stdin(Stdio::null());
        self.command.stdout(Stdio::piped());
        self.command.stderr(Stdio::piped());

        let mut child = self.command.spawn().expect("failed to spawn command");
        let stdout_pipe = child.stdout.take().unwrap();

        let stdout_thread = std::thread::spawn(move || {
            let mut capture = String::new();

            for line in BufReader::new(stdout_pipe).lines() {
                let line = line.unwrap();

                capture.push_str(&line);
                print!("{}\r\n", line);
            }
            capture
        });

        let stderr_pipe = child.stderr.take().unwrap();

        let stderr_thread = std::thread::spawn(move || {
            let mut capture = String::new();

            for line in BufReader::new(stderr_pipe).lines() {
                let line = line.unwrap();

                capture.push_str(&line);
                print!("{line}");
            }
            capture
        });

        let stdout_output = stdout_thread.join().expect("failed to join stdout thread");
        let stderr_output = stderr_thread.join().expect("failed to join stderr thread");
        let exit_status = child.wait()?;

        Ok(Output {
            stdout: stdout_output.into(),
            stderr: stderr_output.into(),
            status: exit_status,
        })
    }
}
