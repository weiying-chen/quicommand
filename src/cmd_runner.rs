use std::{
    io::{BufRead, BufReader},
    process::{Command, Output, Stdio},
};

use crate::utils::escape_backticks;

pub struct CmdRunner {
    pub command: std::process::Command,
}

impl CmdRunner {
    // To-do: this is actually running the command and `run()` is handling the stdout/output.

    pub fn new(command_string: String, input: Option<String>) -> CmdRunner {
        let mut command = Command::new("script");

        command.arg("-qec");

        if let Some(input_str) = input {
            let input_str = escape_backticks(&input_str);

            command.arg(command_string.replace("{}", &input_str));
        } else {
            command.arg(command_string);
        }

        command.arg("/dev/null");
        CmdRunner { command }
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_execute_ok() {
//         let mut cmd_runner = CmdRunner::new("echo test", None);
//         let output = cmd_runner.run().unwrap();
//         let status_string = output.status.code().unwrap();

//         assert_eq!(status_string, 0);
//     }

//     // #[test]
//     // fn test_execute_status() {
//     //     let mut cmd_runner = CmdRunner::new("exit 1", Some(""));
//     //     let mut stdout = Vec::new();

//     //     cmd_runner.run(&mut stdout);

//     //     let stderr_str = String::from_utf8_lossy(&stdout);

//     //     assert!(stderr_str.contains("exit status: 1"));
//     // }

//     // #[test]
//     // fn test_execute_err() {
//     //     let mut cmd_runner = CmdRunner::new("non-existent-command", Some(""));
//     //     let mut stdout = Vec::new();

//     //     cmd_runner.run(&mut stdout);

//     //     let stderr_str = String::from_utf8_lossy(&stdout);

//     //     assert!(stderr_str.contains("command not found"));
//     // }
// }
