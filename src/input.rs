use std::convert::From;
use std::fmt;
use std::io;
use std::io::Write;

use termion::event::Key;

use crate::term_writer::TermWriter;

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
    let mut term_writer = TermWriter::new(input);

    // TODO this side effect maybe shouldn't be here.
    // Or how the input is being written should be more obvious.
    for key in input_keys {
        match key.unwrap() {
            Key::Char('\n') => return term_writer.enter(),
            Key::Esc => return Ok(Input::Exit),
            Key::Char(c) => term_writer.char(stdout, c)?,
            Key::Left => term_writer.left(stdout)?,
            Key::Right => term_writer.right(stdout)?,
            Key::Backspace => term_writer.backspace(stdout)?,
            _ => {}
        }

        stdout.flush().unwrap();
    }

    let input = term_writer.input.trim().to_owned();

    Ok(Input::Text(input))
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
