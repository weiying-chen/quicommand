use std::{
    io::{stdin, BufRead, BufReader, Write},
    process::{Command, Stdio},
    sync::{Arc, Mutex},
};

use termion::{event::Key, input::TermRead};

pub struct CmdRunner {
    pub command: std::process::Command,
}

impl CmdRunner {
    // To-do: this is actually running the command and `run()` is handling the stdout/output.

    pub fn new(command_string: &str, input: Option<&str>) -> CmdRunner {
        let mut command = Command::new("script");

        command.arg("-qec");

        if let Some(input_str) = input {
            command.arg(command_string.replace("{}", input_str));
        } else {
            command.arg(command_string);
        }

        command.arg("/dev/null");

        CmdRunner { command }
    }

    pub fn run<W: Write + Send + 'static>(&mut self, stdout_mutex: Arc<Mutex<Option<W>>>) {
        self.command.stdout(Stdio::piped());
        self.command.stderr(Stdio::piped());

        let mut child = self.command.spawn().expect("failed to spawn command");
        let stdout_pipe = child.stdout.take().unwrap();
        let stdout_reader = BufReader::new(stdout_pipe);
        let stderr_pipe = child.stderr.take().unwrap();
        let stderr_reader = BufReader::new(stderr_pipe);

        let stdout_thread = std::thread::spawn(move || {
            for line in stdout_reader.lines() {
                if let Ok(line) = line {
                    // To-do: Should this be changed to `write!`?
                    print!("{}\r\n", line);
                }
            }
        });

        let stderr_thread = std::thread::spawn(move || {
            for line in stderr_reader.lines() {
                if let Ok(line) = line {
                    print!("{}\r\n", line);
                }
            }
        });

        // let stdout_mutex = Arc::new(Mutex::new(Some(stdout)));
        let stdout_clone = Arc::clone(&stdout_mutex);

        std::thread::spawn(move || {
            let stdout_clone = Arc::clone(&stdout_mutex);

            for c in stdin().keys() {
                match c.unwrap() {
                    // Key::Char(c) => println!("{}\r\n", c),
                    Key::Ctrl('c') => {
                        // write!(stdout, "{}", termion::cursor::Show).unwrap();
                        // stdout.flush().unwrap();

                        //To-do: think of a way of doing this with Termion
                        // Command::new("reset").output().unwrap();

                        let mut stdout_lock = stdout_clone.lock().unwrap();

                        // To-do: create a function called drop()
                        stdout_lock.take();
                        std::process::exit(0);
                    }
                    _ => {}
                }

                let mut stdout_lock = stdout_clone.lock().unwrap();

                stdout_lock.as_mut().unwrap().flush().unwrap();
            }
        });

        // To-do: Is this necessary?
        // stdout.flush().unwrap();

        let status = child.wait().expect("failed to wait for command");

        stdout_thread.join().expect("failed to join stdout thread");
        stderr_thread.join().expect("failed to join stderr thread");

        let mut stdout_lock = stdout_clone.lock().unwrap();
        let stdout = stdout_lock.as_mut().unwrap();

        if status.success() {
            write!(stdout, "Command executed successfully\r\n").unwrap();
        } else {
            write!(
                stdout,
                "Command failed with exit code {}\r\n",
                status.code().unwrap_or(-1)
            )
            .unwrap();
        }

        stdout_lock.take();
        // drop(stdout_lock);

        // spawn_thread.join().unwrap();
    }

    pub fn run_old(&mut self, stdout: &mut impl Write) {
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_execute_ok() {
//         let mut cmd_runner = CmdRunner::new("echo {}", Some("hello"));
//         let mut stdout = Vec::new();

//         cmd_runner.run(&mut stdout);

//         let stdout_str = String::from_utf8(stdout).unwrap();

//         assert_eq!(stdout_str.trim(), "hello");
//     }

//     #[test]
//     fn test_execute_status() {
//         let mut cmd_runner = CmdRunner::new("exit 1", Some(""));
//         let mut stdout = Vec::new();

//         cmd_runner.run(&mut stdout);

//         let stderr_str = String::from_utf8_lossy(&stdout);

//         assert!(stderr_str.contains("exit status: 1"));
//     }

//     #[test]
//     fn test_execute_err() {
//         let mut cmd_runner = CmdRunner::new("non-existent-command", Some(""));
//         let mut stdout = Vec::new();

//         cmd_runner.run(&mut stdout);

//         let stderr_str = String::from_utf8_lossy(&stdout);

//         assert!(stderr_str.contains("command not found"));
//     }
// }
