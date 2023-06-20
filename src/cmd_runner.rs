use std::{
    io::{BufRead, BufReader},
    process::{Command, Output, Stdio},
};

pub struct CmdRunner {
    pub command: std::process::Command,
}

impl CmdRunner {
    // To-do: this is actually running the command and `run()` is handling the stdout/output.

    pub fn new(command_string: String, input: Option<String>) -> CmdRunner {
        let mut command = Command::new("script");

        command.arg("-qec");

        if let Some(input_str) = input {
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

    pub fn run_with_output(&mut self) -> Result<(String, String), std::io::Error> {
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
                println!("{line}");
            }
            capture
        });

        let stdout_output = stdout_thread.join().expect("failed to join stdout thread");
        let stderr_output = stderr_thread.join().expect("failed to join stderr thread");

        // println!("stdout_output: {:?}", stdout_output);
        // println!("stderr_output: {:?}", stderr_output);

        Ok((stdout_output, stderr_output))
    }

    // pub fn run_old<W: Write + Send + 'static>(&mut self, stdout_mutex: Arc<Mutex<Option<W>>>) {
    //     // This prevents the loop from running forever if you press a key other chan Ctrl + C.
    //     // self.command.stdin(Stdio::null());
    //     // self.command.stdout(Stdio::piped());
    //     // self.command.stderr(Stdio::piped());

    //     let mut child = self.command.spawn().expect("failed to spawn command");

    //     // child.wait().expect("editor exited with a non-zero exit");

    //     //     let stdout_pipe = child.stdout.take().unwrap();
    //     //     let stdout_reader = BufReader::new(stdout_pipe);
    //     //     let stderr_pipe = child.stderr.take().unwrap();
    //     //     let stderr_reader = BufReader::new(stderr_pipe);
    //     //     let stdout_clone = Arc::clone(&stdout_mutex);

    //     //     let stdout_thread = std::thread::spawn(move || {
    //     //         for line in stdout_reader.lines() {
    //     //             if let Ok(line) = line {
    //     //                 // To-do: Should this be changed to `write!`?
    //     //                 let mut stdout_lock = stdout_clone.lock().unwrap();
    //     //                 let stdout = stdout_lock.as_mut().unwrap();

    //     //                 write!(stdout, "{}\r\n", line).unwrap();
    //     //             }
    //     //         }
    //     //     });

    //     //     let stdout_clone = Arc::clone(&stdout_mutex);

    //     //     let stderr_thread = std::thread::spawn(move || {
    //     //         for line in stderr_reader.lines() {
    //     //             if let Ok(line) = line {
    //     //                 // print!("{}\r\n", line);
    //     //                 let mut stdout_lock = stdout_clone.lock().unwrap();
    //     //                 let stdout = stdout_lock.as_mut().unwrap();

    //     //                 write!(stdout, "{}\r\n", line).unwrap();
    //     //             }
    //     //         }
    //     //     });

    //     let should_exit = Arc::new(Mutex::new(false));
    //     let should_exit_clone = Arc::clone(&should_exit);
    //     let mut stdin = termion::async_stdin().keys();

    //     loop {
    //         match child.try_wait() {
    //             Ok(Some(_)) => {
    //                 print!("Child process has exited\r\n");
    //                 *should_exit_clone.lock().unwrap() = true;
    //                 break;
    //             }

    //             Ok(None) => {
    //                 if *should_exit_clone.lock().unwrap() {
    //                     print!("Killing the process...\r\n");

    //                     if let Err(e) = child.kill() {
    //                         eprint!("Failed to kill child process: {}", e);
    //                     }

    //                     print!("Process killed!\r\n");
    //                     break;
    //                 }

    //                 let input = stdin.next();

    //                 if let Some(Ok(key)) = input {
    //                     // print!("Key pressed!\r\n");

    //                     match key {
    //                         Key::Ctrl('c') => {
    //                             *should_exit_clone.lock().unwrap() = true;
    //                         }
    //                         Key::Char(_) => {
    //                             continue;
    //                         }
    //                         _ => {}
    //                     }

    //                     // let mut stdout_lock = stdout_clone.lock().unwrap();

    //                     // stdout_lock.as_mut().unwrap().flush().unwrap();
    //                 }

    //                 // print!("Still running...\r\n");

    //                 // let mut stdout_lock = stdout_clone.lock().unwrap();

    //                 // stdout_lock.as_mut().unwrap().flush().unwrap();
    //             }

    //             Err(e) => {
    //                 eprint!("Error while waiting for child process: {}", e);
    //                 break;
    //             }
    //         }

    //         // Delay between polling attempts
    //         std::thread::sleep(Duration::from_millis(100));
    //     }

    //     //     // println!("Before join!");

    //     //     stdout_thread.join().expect("failed to join stdout thread");
    //     //     stderr_thread.join().expect("failed to join stderr thread");
    //     //     // handle.join().unwrap();

    //     //     // let mut stdout_lock = stdout_clone.lock().unwrap();
    //     //     // let stdout = stdout_lock.as_mut().unwrap();

    //     //     // write!(stdout, "Command executed successfully\r\n").unwrap();
    //     // }

    //     // // let mut stdout_lock = stdout_clone.lock().unwrap();
    //     // // let stdout = stdout_lock.as_mut().unwrap();

    //     // // if status.success() {
    //     // //     write!(stdout, "Command executed successfully\r\n").unwrap();
    //     // // } else {
    //     // //     write!(
    //     // //         stdout,
    //     // //         "Command failed with exit code {}\r\n",
    //     // //         status.code().unwrap_or(-1)
    //     // //     )
    //     // //     .unwrap();
    // }
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
