use std::{
    io::{BufRead, BufReader},
    process::{Command, Output, Stdio},
    str::FromStr,
};

use crate::utils::starts_with_any;

pub enum CmdType {
    Interactive,
    Script,
}

pub struct CmdRunner {
    pub cmd: Command,
    pub cmd_type: CmdType,
}

impl FromStr for CmdType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if starts_with_any(s, &["hx", "vi", "fzf"]) {
            Ok(CmdType::Interactive)
        } else {
            Ok(CmdType::Script)
        }
    }
}

impl CmdRunner {
    pub fn new(cmd_str: &str) -> CmdRunner {
        let mut cmd = Command::new("script");

        cmd.arg("-qec").arg(cmd_str).arg("/dev/null");

        let cmd_type = CmdType::from_str(cmd_str).unwrap();

        CmdRunner { cmd, cmd_type }
    }

    pub fn run(&mut self) -> Result<Output, std::io::Error> {
        let child = self.cmd.spawn().expect("failed to spawn command");
        let output = child.wait_with_output()?;

        Ok(output)
    }

    pub fn run_with_output(&mut self) -> Result<Output, std::io::Error> {
        // This prevents the output from becoming messed up in tests.
        self.cmd.stdin(Stdio::null());
        self.cmd.stdout(Stdio::piped());
        self.cmd.stderr(Stdio::piped());

        let mut child = self.cmd.spawn().expect("failed to spawn command");
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
