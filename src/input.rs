use std::convert::From;
use std::fmt;
use std::io;
use std::io::Write;

use termion::event::Key;

use crate::term_writer::TermCursor;
use crate::term_writer::TermWriter;

#[derive(Debug, PartialEq)]
pub enum Input {
    Text(String),
    None,
    Cancel,
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
            InputError::EmptyString => write!(f, "Input was empty"),
            InputError::IoError(e) => write!(f, "I/O Error: {}", e),
        }
    }
}

// This function returns input based on keys
pub fn input_from_keys<T: TermCursor + Write>(
    input_keys: impl Iterator<Item = Result<Key, io::Error>>,
    stdout: &mut T,
) -> Result<Input, InputError> {
    let input = String::new();
    let mut term_writer = TermWriter::new(input, stdout);

    for key in input_keys {
        match key.unwrap() {
            Key::Char('\n') => return term_writer.enter(),
            Key::Esc => return Ok(Input::Cancel),
            Key::Char(c) => term_writer.char(c)?,
            Key::Left => term_writer.left()?,
            Key::Right => term_writer.right()?,
            Key::Backspace => term_writer.backspace()?,
            _ => {}
        }

        term_writer.stdout.flush().unwrap();
    }

    let input = term_writer.input.trim().to_owned();

    Ok(Input::Text(input))
}
