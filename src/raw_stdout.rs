use crate::term_writer::TermCursor;
use std::io::Write;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct RawStdout {
    buffer: RawTerminal<std::io::Stdout>,
}

impl RawStdout {
    pub fn new() -> std::io::Result<Self> {
        let buffer = std::io::stdout().into_raw_mode()?;
        Ok(Self { buffer })
    }
}

impl Write for RawStdout {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buffer.flush()
    }
}

impl TermCursor for RawStdout {
    fn write_term(&mut self, fmt: std::fmt::Arguments) -> std::io::Result<()> {
        std::io::Write::write_fmt(self, fmt)
    }

    fn get_cursor_pos(&mut self) -> Result<(u16, u16), std::io::Error> {
        termion::cursor::DetectCursorPos::cursor_pos(self)
    }
}
