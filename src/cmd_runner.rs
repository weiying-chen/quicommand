use std::{
    io::{self, stderr, stdout, BufRead, BufReader, Write},
    process::{Command, Output, Stdio},
    str::FromStr,
    thread::JoinHandle,
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

fn tee<R, W>(reader: R, mut writer: W) -> JoinHandle<io::Result<String>>
where
    R: BufRead + Send + 'static,
    W: Write + Send + 'static,
{
    std::thread::spawn(move || {
        let mut capture = String::new();

        for line in reader.lines() {
            let line = line?;

            capture.push_str(&line);
            writer.write_all(line.as_bytes())?;
            writer.write_all(b"\r\n")?;
            writer.flush()?;
        }

        Ok(capture)
    })
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
        let stdout_thread = tee(BufReader::new(stdout_pipe), stdout());
        let stderr_pipe = child.stderr.take().unwrap();
        let stderr_thread = tee(BufReader::new(stderr_pipe), stderr());
        let stdout_output = stdout_thread.join().expect("failed to join stdout thread");
        let stderr_output = stderr_thread.join().expect("failed to join stderr thread");
        let exit_status = child.wait()?;

        Ok(Output {
            stdout: stdout_output?.into(),
            stderr: stderr_output?.into(),
            status: exit_status,
        })
    }
}
