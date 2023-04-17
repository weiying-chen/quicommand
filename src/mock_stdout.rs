use std::io::Write;

#[derive(Default, Debug)]
pub struct MockStdout {
    pub buffer: Vec<u8>,
    pub cursor_pos: (u16, u16),
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
