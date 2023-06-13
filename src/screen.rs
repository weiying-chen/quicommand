use crate::term_writer::TermCursor;
use std::io::Write;

pub struct Screen<T: TermCursor + Write> {
    pub stdout: T,
}

impl<T: TermCursor + Write> Screen<T> {
    pub fn new(stdout: T) -> Self {
        Screen { stdout }
    }

    //To-do: maybe functions like these should belong to `TermWriter`?
    pub fn add_newline(&mut self) {
        self.stdout.write_term(format_args!("\r\n")).unwrap();
    }

    pub fn show_cursor(&mut self) {
        self.stdout
            .write_term(format_args!("{}", termion::cursor::Show))
            .unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn show_prompt(&mut self, message: &str) {
        self.stdout
            .write_term(format_args!("{}\r\n", message))
            .unwrap();
    }

    pub fn show_menu(&mut self, items: &[String]) {
        for item in items {
            self.stdout
                .write_term(format_args!("{}\r\n", item))
                .unwrap();
        }
    }

    pub fn clear_all(&mut self) {
        self.stdout
            .write_term(format_args!(
                "{}{}{}",
                termion::clear::All,
                termion::cursor::Goto(1, 1),
                termion::cursor::Hide,
            ))
            .unwrap();
    }
}
