use std::convert::From;
use std::fmt;
use std::io;

pub enum Input {
    Text(String),
    Exit,
}

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
