use crate::input::{Input, InputError};

struct Position {
    x: u16,
    y: u16,
}

// CursorPos

pub trait TermCursor {
    fn write_term(&mut self, fmt: std::fmt::Arguments) -> std::io::Result<()>;
    fn get_cursor_pos(&mut self) -> Result<(u16, u16), std::io::Error>;
}

// TermWriter

pub struct TermWriter<'a, T: TermCursor> {
    pub input: String,
    pub stdout: &'a mut T,
    cursor_pos: Position,
}

impl<'a, C: TermCursor> TermWriter<'a, C> {
    pub fn new(input: String, stdout: &'a mut C) -> Self {
        Self {
            input,
            stdout,
            cursor_pos: Position { x: 1, y: 3 },
        }
    }

    pub fn enter(self) -> Result<Input, InputError> {
        if self.input.trim().is_empty() {
            Err(InputError::EmptyString)
        } else {
            Ok(Input::Text(self.input))
        }
    }

    pub fn left(&mut self) -> Result<(), InputError> {
        self.stdout
            .write_term(format_args!("{}", termion::cursor::Left(1)))?;

        let cursor_pos = self.stdout.get_cursor_pos()?;

        self.cursor_pos.x = cursor_pos.0;

        Ok(())
    }

    pub fn right(&mut self) -> Result<(), InputError> {
        if self.cursor_pos.x <= self.input.len() as u16 {
            self.stdout
                .write_term(format_args!("{}", termion::cursor::Right(1)))?;

            let cursor_pos = self.stdout.get_cursor_pos()?;

            self.cursor_pos.x = cursor_pos.0;
        }

        Ok(())
    }

    pub fn backspace(&mut self) -> Result<(), InputError> {
        if self.cursor_pos.x > 1 {
            self.cursor_pos.x -= 1;
            self.input.remove((self.cursor_pos.x - 1).into());

            let cursor_pos = self.stdout.get_cursor_pos()?;

            self.cursor_pos.y = cursor_pos.1;

            self.stdout.write_term(format_args!(
                "{}{}{}",
                termion::cursor::Goto(1, self.cursor_pos.y),
                termion::clear::CurrentLine,
                self.input,
            ))?;

            self.stdout.write_term(format_args!(
                "{}",
                termion::cursor::Goto(self.cursor_pos.x, self.cursor_pos.y)
            ))?;
        }

        Ok(())
    }

    pub fn char(&mut self, c: char) -> Result<(), InputError> {
        let bytes = vec![c as u8];
        std::str::from_utf8(&bytes)
            .map_err(|_| InputError::NotUTF8(bytes.clone()))
            .and_then(|_| {
                self.input.insert((self.cursor_pos.x - 1).into(), c);

                let cursor_pos = self.stdout.get_cursor_pos()?;

                self.cursor_pos.y = cursor_pos.1;

                self.stdout.write_term(format_args!(
                    "{}{}{}",
                    termion::cursor::Goto(1, self.cursor_pos.y),
                    termion::clear::CurrentLine,
                    self.input,
                ))?;

                self.stdout.write_term(format_args!(
                    "{}",
                    termion::cursor::Goto(self.cursor_pos.x + 1, self.cursor_pos.y)
                ))?;

                let cursor_pos = self.stdout.get_cursor_pos()?;

                self.cursor_pos.x = cursor_pos.0;

                Ok(())
            })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    // Stdout

    #[derive(Default, Debug)]
    struct Stdout {
        buffer: Vec<u8>,
        cursor_pos: (u16, u16),
    }

    impl Stdout {
        pub fn new() -> Self {
            let buffer = Vec::new();

            Self {
                buffer,
                cursor_pos: (1, 1),
            }
        }
    }

    impl Write for Stdout {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.buffer.write(buf)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.buffer.flush()
        }
    }

    impl TermCursor for Stdout {
        fn write_term(&mut self, fmt: std::fmt::Arguments) -> std::io::Result<()> {
            const CURSOR_LEFT: &str = "\u{1b}[1C";
            const CLEAR_ALL: &str = "\u{1b}[2K";

            match fmt.to_string().as_str() {
                CURSOR_LEFT => {
                    self.cursor_pos.0 += 1;
                }
                s if s.contains(CLEAR_ALL) => {
                    self.cursor_pos.0 -= 1;
                }
                _ => {}
            }

            Ok(())
        }

        fn get_cursor_pos(&mut self) -> Result<(u16, u16), std::io::Error> {
            Ok(self.cursor_pos)
        }
    }

    #[test]
    fn test_right() {
        let input = "abc".to_string();
        let mut stdout = Stdout::new();
        let mut term_writer = TermWriter::new(input, &mut stdout);

        for _ in 0..3 {
            term_writer.right().unwrap();
        }

        let cursor_pos = term_writer.stdout.get_cursor_pos().unwrap();

        term_writer.cursor_pos.x = cursor_pos.0;
        assert_eq!(term_writer.cursor_pos.x, 4);

        term_writer.right().unwrap();
        assert_eq!(term_writer.cursor_pos.x, 4);
    }

    #[test]
    fn test_backspace() {
        let input = "abc".to_string();
        let mut stdout = Stdout::new();
        let mut term_writer = TermWriter::new(input, &mut stdout);

        term_writer.right().unwrap();
        term_writer.backspace().unwrap();
        assert_eq!(term_writer.input, "bc");
        term_writer.backspace().unwrap();
        assert_eq!(term_writer.input, "bc");
    }

    #[test]
    fn test_char() {
        let input = "ab".to_string();
        let mut stdout = Stdout::new();
        let mut term_writer = TermWriter::new(input, &mut stdout);

        for _ in 0..3 {
            term_writer.right().unwrap();
        }
        term_writer.char('c').unwrap();
        assert_eq!(term_writer.input, "abc");
    }
}
