use crate::cmd_runner::{CmdRunner, CmdType};
use crate::input;
use crate::input::{Input, InputError};
use crate::keymap::Keymap;
use crate::screen::Screen;
use crate::term_writer::TermCursor;
use crate::utils::escape_backticks;
use std::io::Write;
use termion::event::Key;

#[derive(Debug, PartialEq)]
pub enum Process {
    Output(std::process::Output),
    Exit,
}

pub struct Step<T: TermCursor + Write> {
    pub screen: Screen<T>,
}

impl<T: TermCursor + Write> Step<T> {
    pub fn new(screen: Screen<T>) -> Self {
        Step { screen }
    }

    pub fn show_select_cmd(&mut self, keymaps: &[Keymap]) {
        self.screen.clear_all();
        self.screen.show_prompt("Please select a command:");

        let menu_items: Vec<String> = keymaps
            .iter()
            .map(|keymap| format!("{}  {}", keymap.key, keymap.description))
            .collect();

        self.screen.show_menu(&menu_items);
    }

    pub fn input_from_prompt(
        &mut self,
        prompt: Option<&str>,
        stdin: impl Iterator<Item = Result<Key, std::io::Error>>,
    ) -> Result<Input, InputError> {
        match prompt {
            Some(_) => {
                self.screen.show_prompt(&prompt.unwrap());
                self.screen.show_cursor();

                let input = input::input_from_keys(stdin, &mut self.screen.stdout)?;
                // To-do: move `escape_backtips` to here?

                Ok(input)
            }
            None => Ok(Input::None),
        }
    }

    pub fn process_input(
        mut self,
        result: Result<Input, InputError>,
        keymap: &Keymap,
    ) -> Result<Process, InputError> {
        match result {
            Ok(Input::Text(i)) => {
                // Because the input doesn't start a newline
                self.screen.add_newline();
                self.screen.show_cursor();
                drop(self.screen.stdout);

                let input = escape_backticks(&i);
                let keymap_cmd = keymap.cmd.replace("{}", &input);

                let mut cmd_runner = CmdRunner::new(&keymap_cmd);
                let output = cmd_runner.run_with_output().unwrap();

                Ok(Process::Output(output))
            }
            Ok(Input::None) => {
                self.screen.show_cursor();
                drop(self.screen.stdout);

                let mut cmd_runner = CmdRunner::new(&keymap.cmd);

                let output = match cmd_runner.cmd_type {
                    CmdType::Interactive => cmd_runner.run().unwrap(),
                    CmdType::Script => cmd_runner.run_with_output().unwrap(),
                };

                Ok(Process::Output(output))
            }
            Ok(Input::Cancel) => {
                self.screen.add_newline();
                Ok(Process::Exit)
            }
            Err(e) => {
                self.screen
                    .stdout
                    .write_term(format_args!("Invalid input: {}\r\n", e))
                    .unwrap();
                Err(e)
            }
        }
    }
}
