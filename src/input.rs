use std::convert::From;
use std::fmt;
use std::io;
use std::io::Write;

use termion::event::Key;

use crate::key_handler::KeyHandler;

#[derive(Debug, PartialEq)]
pub enum Input {
    Text(String),
    Exit,
}

#[derive(Debug)]
pub enum InputError {
    NotUTF8(Vec<u8>),
    EmptyString,
    IoError(io::Error),
}

impl From<io::Error> for InputError {
    fn from(error: io::Error) -> Self {
        InputError::IoError(error)
    }
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputError::NotUTF8(bytes) => write!(
                f,
                "Input contained non-UTF8 bytes: {:?}",
                bytes
                    .iter()
                    .map(|b| format!("0x{:X}", b))
                    .collect::<Vec<_>>()
            ),
            InputError::EmptyString => write!(f, "Input was empty."),
            InputError::IoError(e) => write!(f, "I/O Error: {}", e),
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

pub fn handle_input(input_text: Result<Input, InputError>, stdout: &mut impl Write) -> String {
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

    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_error_display() {
        let not_utf8_bytes = vec![0xFF, 0xFE];
        let err = InputError::NotUTF8(not_utf8_bytes);
        assert_eq!(
            format!("{}", err),
            "Input contained non-UTF8 bytes: [\"0xFF\", \"0xFE\"]"
        );

        let err = InputError::EmptyString;
        assert_eq!(format!("{}", err), "Input was empty.");

        let io_err = io::Error::new(io::ErrorKind::Other, "test error");
        let err = InputError::IoError(io_err);
        assert_eq!(format!("{}", err), "I/O Error: test error");
    }
}
