use std::{
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
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
        // This prevents the loop from running forever if you press a key other chan Ctrl + C.
        self.command.stdin(Stdio::null());
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

        let should_exit = Arc::new(Mutex::new(false));
        let should_exit_clone = Arc::clone(&should_exit);
        let mut stdin = termion::async_stdin().keys();
        let stdout_clone = Arc::clone(&stdout_mutex);

        loop {
            match child.try_wait() {
                Ok(Some(_)) => {
                    // print!("Child process has exited\r\n");
                    *should_exit_clone.lock().unwrap() = true;
                    break;
                }

                Ok(None) => {
                    if *should_exit_clone.lock().unwrap() {
                        print!("Killing the process...\r\n");

                        if let Err(e) = child.kill() {
                            eprint!("Failed to kill child process: {}", e);
                        }

                        print!("Process killed!\r\n");
                        break;
                    }

                    let input = stdin.next();

                    if let Some(Ok(key)) = input {
                        // print!("Key pressed!\r\n");

                        match key {
                            Key::Ctrl('c') => {
                                *should_exit_clone.lock().unwrap() = true;
                            }
                            Key::Char(_) => {
                                continue;
                            }
                            _ => {}
                        }

                        // let mut stdout_lock = stdout_clone.lock().unwrap();

                        // stdout_lock.as_mut().unwrap().flush().unwrap();
                    }

                    // print!("Still running...\r\n");

                    // let mut stdout_lock = stdout_clone.lock().unwrap();

                    // stdout_lock.as_mut().unwrap().flush().unwrap();
                }

                Err(e) => {
                    eprint!("Error while waiting for child process: {}", e);
                    break;
                }
            }

            // Delay between polling attempts
            std::thread::sleep(Duration::from_millis(100));
        }

        // println!("Before join!");

        stdout_thread.join().expect("failed to join stdout thread");
        stderr_thread.join().expect("failed to join stderr thread");
        // handle.join().unwrap();
    }

    // let mut stdout_lock = stdout_clone.lock().unwrap();
    // let stdout = stdout_lock.as_mut().unwrap();

    // if status.success() {
    //     write!(stdout, "Command executed successfully\r\n").unwrap();
    // } else {
    //     write!(
    //         stdout,
    //         "Command failed with exit code {}\r\n",
    //         status.code().unwrap_or(-1)
    //     )
    //     .unwrap();
    // }
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
