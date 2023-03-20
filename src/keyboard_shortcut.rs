use std::{
    io::{stdin, Write},
    process::Command,
};

use termion::{event::Key, input::TermRead};

use crate::{
    input::{Input, InputError},
    key_handler::KeyHandler,
};

pub struct KeyboardShortcut {
    pub key: char,
    pub description: &'static str,
    pub command: &'static str,
    pub input_placeholder: &'static str,
}

impl KeyboardShortcut {
    pub fn execute_command(&self, stdout: &mut impl Write) {
        write!(stdout, "Enter commit message: ").unwrap();
        stdout.flush().unwrap();

        let input = match get_input(stdout) {
            Ok(Input::Text(i)) => i,
            Ok(Input::Exit) => {
                write!(stdout, "\r\n").unwrap();
                return;
            }
            Err(e) => {
                write!(stdout, "\r\nInvalid input: {}\r\n", e).unwrap();
                return;
            }
        };

        let command = self.command.replace(self.input_placeholder, &input);

        // This combination makes commands print colors.
        let output = Command::new("script")
            .arg("-qec")
            .arg(command)
            .arg("/dev/null")
            .output();

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

                    write!(stdout, "stdout: {}\r\n", stderr_str.trim()).unwrap();
                    write!(stdout, "stderr: {}\r\n", stdout_str.trim()).unwrap();
                }
            }
            Err(e) => {
                write!(stdout, "Error executing command: {:?}\r\n", e).unwrap();
            }
        }
    }
}

fn get_input(stdout: &mut impl Write) -> Result<Input, InputError> {
    let input = String::new();
    // let mut y = 1;
    let mut key_handler = KeyHandler::new(input);

    for key in stdin().keys() {
        match key.unwrap() {
            Key::Char('\n') => return key_handler.enter(),
            Key::Esc => return Ok(Input::Exit),
            Key::Char(c) => key_handler.char(stdout, c)?,
            Key::Left => key_handler.left(stdout)?,
            Key::Right => key_handler.right(stdout)?,
            Key::Backspace => key_handler.backspace(stdout)?,
            _ => {}
        }

        stdout.flush().unwrap();
    }

    // This places the output on a new line.
    write!(stdout, "\r\n").unwrap();

    // TODO: maybe a function should return the input instead?
    let input = key_handler.input.trim().to_owned();

    Ok(Input::Text(input))
}
