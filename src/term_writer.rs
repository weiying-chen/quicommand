use crate::input::{Input, InputError};
use std::io::Write;
use termion::cursor::DetectCursorPos;

struct Position {
    x: u16,
    y: u16,
}

// CursorPos

pub trait CursorPos {
    fn write_fmt(&mut self, s: &str) -> Result<(), std::io::Error>;
    fn cursor_position(&self) -> Result<(u16, u16), std::io::Error>;
}

// Stdout

#[derive(Debug)]
struct Stdout;

impl CursorPos for Stdout {
    fn write_fmt(&mut self, s: &str) -> Result<(), std::io::Error> {
        // write!(std::io::stdout(), "{}", s)
        Ok(())
    }

    fn cursor_position(&self) -> Result<(u16, u16), std::io::Error> {
        // termion::cursor::DetectCursorPos::cursor_pos()
        Ok((2, 2))
    }
}

// TermWriter

pub struct TermWriter<'a, T: CursorPos> {
    // TODO: Maybe input shouldn't belong to this struct.
    pub input: String,
    stdout: &'a mut T,
    cursor_pos: Position,
}

impl<'a, T: CursorPos> TermWriter<'a, T> {
    pub fn new(input: String, stdout: &'a mut T) -> Self {
        Self {
            input,
            stdout,
            cursor_pos: Position { x: 1, y: 2 },
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
        // write!(self.stdout, "{}", termion::cursor::Left(1))?;

        let cursor_pos = self.stdout.cursor_position()?;

        self.cursor_pos.x = cursor_pos.0;

        Ok(())
    }

    // pub fn right(&mut self, stdout: &mut impl Write) -> Result<(), InputError> {
    //     //TODO: See if can remove these if statements all of the function
    //     // Or check if if statements in functions are okay
    //     if self.cursor_pos.x <= self.input.len() as u16 {
    //         write!(stdout, "{}", termion::cursor::Right(1))?;

    //         let cursor_pos = stdout.cursor_position()?;

    //         self.cursor_pos.x = cursor_pos.0;
    //     }

    //     Ok(())
    // }

    // pub fn backspace(&mut self, stdout: &mut impl Write) -> Result<(), InputError> {
    //     if self.cursor_pos.x > 1 {
    //         self.cursor_pos.x -= 1;
    //         self.input.remove((self.cursor_pos.x - 1).into());

    //         let cursor_pos = stdout.cursor_pos()?;

    //         self.cursor_pos.y = cursor_pos.1;

    //         write!(
    //             stdout,
    //             "{}{}{}",
    //             termion::cursor::Goto(1, self.cursor_pos.y),
    //             termion::clear::CurrentLine,
    //             self.input,
    //         )?;

    //         write!(
    //             stdout,
    //             "{}",
    //             termion::cursor::Goto(self.cursor_pos.x, self.cursor_pos.y)
    //         )?;
    //     }

    //     Ok(())
    // }

    // pub fn char(&mut self, stdout: &mut impl Write, c: char) -> Result<(), InputError> {
    //     let bytes = vec![c as u8];
    //     std::str::from_utf8(&bytes)
    //         .map_err(|_| InputError::NotUTF8(bytes.clone()))
    //         .and_then(|_| {
    //             self.input.insert((self.cursor_pos.x - 1).into(), c);

    //             let cursor_pos = stdout.cursor_pos()?;

    //             self.cursor_pos.y = cursor_pos.1;

    //             write!(
    //                 stdout,
    //                 "{}{}{}",
    //                 termion::cursor::Goto(1, self.cursor_pos.y),
    //                 termion::clear::CurrentLine,
    //                 self.input,
    //             )?;

    //             write!(
    //                 stdout,
    //                 "{}",
    //                 termion::cursor::Goto(self.cursor_pos.x + 1, self.cursor_pos.y)
    //             )?;

    //             let cursor_pos = stdout.cursor_pos()?;

    //             self.cursor_pos.x = cursor_pos.0;

    //             Ok(())
    //         })
    // }
}
