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

pub fn get_input<T: TermCursor + Write>(
    input_keys: impl Iterator<Item = Result<Key, io::Error>>,
    stdout: &mut T,
) -> Result<Input, InputError> {
    let input = String::new();
    let mut term_writer = TermWriter::new(input, stdout);

    for key in input_keys {
        match key.unwrap() {
            Key::Char('\n') => return term_writer.enter(),
            Key::Esc => return Ok(Input::Exit),
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

#[cfg(test)]
mod tests {
    use super::*;

    // Stdout

    #[derive(Default, Debug)]
    struct Stdout {
        cursor_pos: (u16, u16),
    }

    impl Stdout {
        fn new() -> Self {
            Stdout {
                cursor_pos: (1, 1),
                ..Default::default()
            }
        }
    }

    impl TermCursor for Stdout {
        fn write_term(&mut self, fmt: std::fmt::Arguments) -> std::io::Result<()> {
            const INPUT_START: &str = "\u{1b}[2K";

            if fmt.to_string().contains(INPUT_START) {
                self.cursor_pos.0 += 1;
            }

            Ok(())
        }

        fn get_cursor_pos(&mut self) -> Result<(u16, u16), std::io::Error> {
            Ok(self.cursor_pos)
        }
    }

    impl Write for Stdout {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            std::io::Write::write(&mut std::io::stdout(), buf)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            std::io::Write::flush(&mut std::io::stdout())
        }
    }

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

    //To-do: test Left, Right, Esc, and Backspace.
    #[test]
    fn test_get_input() {
        let keys = vec![
            Ok(Key::Char('a')),
            Ok(Key::Char('b')),
            Ok(Key::Char('c')),
            Ok(Key::Char('\n')),
        ];

        let mut stdout = Stdout::new();

        let result = get_input(keys.into_iter(), &mut stdout);

        assert_eq!(result.unwrap(), Input::Text(String::from("abc")));
    }
}
