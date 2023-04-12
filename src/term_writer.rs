use crate::input::{Input, InputError};
// use std::io::Write;
// use termion::cursor::DetectCursorPos;

struct Position {
    x: u16,
    y: u16,
}

// CursorPos

pub trait CursorPos {
    // fn write_fmt(&mut self, fmt: std::fmt::Arguments) -> std::io::Result<()>;
    fn write_term(&mut self, fmt: std::fmt::Arguments) -> std::io::Result<()>;
    fn cursor_position(&mut self) -> Result<(u16, u16), std::io::Error>;
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

pub struct TermWriter<'a, T: CursorPos> {
    // TODO: Maybe input shouldn't belong to this struct.
    pub input: String,
    pub stdout: &'a mut T,
    cursor_pos: Position,
}

impl<'a, C: CursorPos> TermWriter<'a, C> {
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

        let cursor_pos = self.stdout.cursor_position()?;

        self.cursor_pos.x = cursor_pos.0;

        Ok(())
    }

    pub fn right(&mut self) -> Result<(), InputError> {
        //TODO: See if can remove these if statements all of the function
        // Or check if if statements in functions are okay
        if self.cursor_pos.x <= self.input.len() as u16 {
            self.stdout
                .write_term(format_args!("{}", termion::cursor::Right(1)))?;

            let cursor_pos = self.stdout.cursor_position()?;

            self.cursor_pos.x = cursor_pos.0;
        }

        Ok(())
    }

    pub fn backspace(&mut self) -> Result<(), InputError> {
        if self.cursor_pos.x > 1 {
            self.cursor_pos.x -= 1;
            self.input.remove((self.cursor_pos.x - 1).into());

            let cursor_pos = self.stdout.cursor_position()?;

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

                let cursor_pos = self.stdout.cursor_position()?;

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

                let cursor_pos = self.stdout.cursor_position()?;

                self.cursor_pos.x = cursor_pos.0;

                Ok(())
            })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::input::Input;

//     // Create a fake CursorPos implementation that always returns (1, 1) as the cursor position
//     struct FakeCursorPos {}

//     impl CursorPos for FakeCursorPos {
//         fn write_term(&mut self, _fmt: std::fmt::Arguments) -> std::io::Result<()> {
//             Ok(())
//         }

//         fn cursor_position(&mut self) -> Result<(u16, u16), std::io::Error> {
//             Ok((1, 1))
//         }
//     }

//     #[test]
//     fn test_term_writer() {
//         // Test cursor position
//         let mut stdout = FakeCursorPos {};

//         assert_eq!(stdout.cursor_position().unwrap(), (1, 1));

//         // Test text editing
//         let mut writer = TermWriter::new("hello".to_string(), &mut stdout);

//         writer.right().unwrap();
//         writer.char('X').unwrap();
//         writer.left().unwrap();
//         writer.backspace().unwrap();

//         assert_eq!(writer.input, "helo");

//         // Test input validation
//         let mut writer = TermWriter::new("".to_string(), &mut stdout);

//         assert_eq!(writer.enter().unwrap_err(), InputError::EmptyString);

//         let mut writer = TermWriter::new("hello".to_string(), &mut stdout);

//         assert_eq!(writer.enter().unwrap(), Input::Text("hello".to_string()));
//     }
// }
