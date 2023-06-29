use crate::term_writer::TermCursor;
use std::io::Write;

#[derive(Debug)]
pub struct MockStdout {
    pub buffer: Vec<u8>,
    cursor_pos: (u16, u16),
}

impl MockStdout {
    pub fn new() -> Self {
        let buffer = Vec::new();

        Self {
            buffer,
            cursor_pos: (1, 1),
        }
    }
}

impl Write for MockStdout {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buffer.flush()
    }
}

impl TermCursor for MockStdout {
    fn write_term(&mut self, fmt: std::fmt::Arguments) -> std::io::Result<()> {
        const INPUT_START: &str = "\u{1b}[2K";

        if fmt.to_string().contains(INPUT_START) {
            self.cursor_pos.0 += 1;
        }

        self.write_fmt(fmt).unwrap();

        Ok(())
    }

    fn get_cursor_pos(&mut self) -> Result<(u16, u16), std::io::Error> {
        Ok(self.cursor_pos)
    }
}
