use crate::cmd_runner::CmdRunner;
use crate::input;
use crate::input::{Input, InputError};
use crate::keymap::Keymap;
use crate::term_writer::TermCursor;
use std::io::Write;
use termion::event::Key;

pub struct Screen<T: TermCursor + Write> {
    pub stdout: T,
}

impl<T: TermCursor + Write> Screen<T> {
    //To-do: maybe functions like these should belong to `TermWriter`?
    pub fn new(stdout: T) -> Self {
        Screen { stdout }
    }

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

    pub fn input_from_prompt(
        &mut self,
        keymap: &Keymap,
        stdin: impl Iterator<Item = Result<Key, std::io::Error>>,
    ) -> Result<Input, InputError> {
        match keymap.prompt {
            Some(_) => {
                self.show_prompt(&keymap.prompt.clone().unwrap());
                self.show_cursor();

                let input = input::input_from_keys(stdin, &mut self.stdout)?;

                Ok(input)
            }
            None => Ok(Input::None),
        }
    }

    pub fn handle_input_result(mut self, result: Result<Input, InputError>, keymap: &Keymap) {
        match result {
            Ok(Input::Text(i)) => {
                // Because the input doesn't start a newline
                self.add_newline();
                self.show_cursor();
                drop(self.stdout);

                // To-do: `command should` return a result.
                // To-do: The cursor is shown previously in prompt_input.
                let mut command = CmdRunner::new(keymap.command.clone(), Some(i));
                // let stdout_mutex = Arc::new(Mutex::new(Some(stdout)));

                command.run().unwrap();
            }
            Ok(Input::None) => {
                self.show_cursor();
                drop(self.stdout);

                let mut command = CmdRunner::new(keymap.command.clone(), None);

                command.run().unwrap();
            }
            Ok(Input::Exit) => {
                self.add_newline();
            }
            Err(e) => {
                self.stdout
                    .write_term(format_args!("Invalid input: {}\r\n", e))
                    .unwrap();
            }
        }
    }
}
