use crate::input::{Input, InputError};
// use std::io::Write;
// use termion::cursor::DetectCursorPos;

struct Position {
    x: u16,
    y: u16,
}

// CursorPos

pub trait TermCursor {
    // fn write_fmt(&mut self, fmt: std::fmt::Arguments) -> std::io::Result<()>;
    fn write_term(&mut self, fmt: std::fmt::Arguments) -> std::io::Result<()>;
    fn get_cursor_pos(&mut self) -> Result<(u16, u16), std::io::Error>;
}

// Stdout

// #[derive(Debug)]
// struct Stdout {}

// impl Write for Stdout {
//     fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//         std::io::Write::write(&mut std::io::stdout(), buf)
//     }

//     fn flush(&mut self) -> std::io::Result<()> {
//         std::io::Write::flush(&mut std::io::stdout())
//     }
// }

// impl CursorPos for Stdout {
//     fn write_term(&mut self, fmt: std::fmt::Arguments) -> std::io::Result<()> {
//         std::io::Write::write_fmt(self, fmt)
//     }

//     fn cursor_position(&mut self) -> Result<(u16, u16), std::io::Error> {
//         termion::cursor::DetectCursorPos::cursor_pos(self)
//     }
// }

// TermWriter

pub struct TermWriter<'a, T: TermCursor> {
    // TODO: Maybe input shouldn't belong to this struct.
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
        //TODO: See if can remove these if statements all of the function
        // Or check if if statements in functions are okay
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
            const CURSOR_LEFT: &str = "\u{1b}[1C";

            println!("===");
            println!("FMT: {:?}", fmt.to_string());
            println!("===");

            // To-do: use match here instead:
            if fmt.to_string() == CURSOR_LEFT {
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
