use crate::input::{Input, InputError};
use std::io::Write;
use termion::cursor::DetectCursorPos;

struct Position {
    x: u16,
    y: u16,
}

pub struct KeyHandler {
    pub input: String,
    cursor_pos: Position,
}

const CURSOR_REQUIRED: &str = "Terminal must support cursor";

impl KeyHandler {
    pub fn new(input: String) -> Self {
        Self {
            input,
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

    pub fn left(&mut self, stdout: &mut impl Write) {
        write!(stdout, "{}", termion::cursor::Left(1)).unwrap();

        let cursor_pos = stdout.cursor_pos().expect(CURSOR_REQUIRED);

        self.cursor_pos.x = cursor_pos.0;
    }

    pub fn right(&mut self, stdout: &mut impl Write) {
        //TODO: See if can remove these if statements all of the function
        // Or check if if statements in functions are okay
        if self.cursor_pos.x <= self.input.len() as u16 {
            write!(stdout, "{}", termion::cursor::Right(1)).unwrap();

            let cursor_pos = stdout.cursor_pos().expect(CURSOR_REQUIRED);

            self.cursor_pos.x = cursor_pos.0;
        }
    }

    pub fn backspace(&mut self, stdout: &mut impl Write) {
        if self.cursor_pos.x > 1 {
            self.cursor_pos.x -= 1;
            self.input.remove((self.cursor_pos.x - 1).into());

            let cursor_pos = stdout.cursor_pos().expect(CURSOR_REQUIRED);

            self.cursor_pos.y = cursor_pos.1;

            write!(
                stdout,
                "{}{}{}",
                termion::cursor::Goto(1, self.cursor_pos.y),
                termion::clear::CurrentLine,
                self.input,
            )
            .unwrap();

            write!(
                stdout,
                "{}",
                termion::cursor::Goto(self.cursor_pos.x, self.cursor_pos.y)
            )
            .unwrap();
        }
    }

    pub fn char(&mut self, stdout: &mut impl Write, c: char) -> Result<(), InputError> {
        let bytes = vec![c as u8];
        std::str::from_utf8(&bytes)
            .map_err(|_| InputError::NotUTF8(bytes.clone()))
            .and_then(|_| {
                self.input.insert((self.cursor_pos.x - 1).into(), c);

                let cursor_pos = stdout.cursor_pos().expect(CURSOR_REQUIRED);

                self.cursor_pos.y = cursor_pos.1;

                write!(
                    stdout,
                    "{}{}{}",
                    termion::cursor::Goto(1, self.cursor_pos.y),
                    termion::clear::CurrentLine,
                    self.input,
                )
                .unwrap();

                write!(
                    stdout,
                    "{}",
                    termion::cursor::Goto(self.cursor_pos.x + 1, self.cursor_pos.y)
                )
                .unwrap();

                let cursor_pos = stdout.cursor_pos().expect(CURSOR_REQUIRED);

                self.cursor_pos.x = cursor_pos.0;

                Ok(())
            })
    }
}
