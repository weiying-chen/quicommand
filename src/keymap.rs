use std::{
    io::{self, Write},
    process::Command,
};

use termion::event::Key;

use crate::{
    input::{Input, InputError},
    key_handler::KeyHandler,
};

pub struct Keymap {
    pub key: char,
    pub description: &'static str,
    pub command: &'static str,
    pub input_placeholder: &'static str,
}

//  `get_input` should be extracted out of `execute_command()`.
impl Keymap {
    pub fn generate_command(
        &self,
        input_text: Result<Input, InputError>,
        stdout: &mut impl Write,
    ) -> String {
        // TODO: Maybe input_keys should be a struct field?
        let input = match input_text {
            Ok(Input::Text(i)) => i,
            Ok(Input::Exit) => {
                write!(stdout, "\r\n").unwrap();
                // TODO: Maybe there's a better way of handling this?
                std::process::exit(0);
            }
            Err(e) => {
                write!(stdout, "\r\nInvalid input: {}\r\n", e).unwrap();
                std::process::exit(1);
            }
        };

        self.command.replace(self.input_placeholder, &input)
    }

    pub fn execute_command(&self, command: String, stdout: &mut impl Write) {
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

// TODO: maybe this function shouldn't be in this file.
pub fn get_input(
    input_keys: impl Iterator<Item = Result<Key, io::Error>>,
    stdout: &mut impl Write,
) -> Result<Input, InputError> {
    let input = String::new();
    let mut key_handler = KeyHandler::new(input);

    for key in input_keys {
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

    let input = key_handler.input.trim().to_owned();

    Ok(Input::Text(input))
}

//TODO: test Left, Right, Esc, and Backspace.
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::stdout;
    use termion::raw::IntoRawMode;

    #[test]
    fn test_get_input() {
        let keys = vec![
            Ok(Key::Char('h')),
            Ok(Key::Char('e')),
            Ok(Key::Char('l')),
            Ok(Key::Char('l')),
            Ok(Key::Char('o')),
            Ok(Key::Char('\n')),
        ];

        let mut stdout = stdout().into_raw_mode().unwrap();
        let result = get_input(keys.into_iter(), &mut stdout);

        assert_eq!(result.unwrap(), Input::Text(String::from("hello")));
    }
}
