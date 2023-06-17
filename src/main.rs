use keymap::cmd_runner::CmdRunner;
use keymap::input;
use keymap::input::{Input, InputError};
use keymap::keymap::Keymap;
use keymap::screen::Screen;
use keymap::term_writer::TermCursor;
use std::io::{stdin, Write};
// use std::sync::{Arc, Mutex};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

//To-do: move this into its own file.

struct RawStdout {
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

#[derive(Debug, PartialEq)]
pub enum Process {
    Output(std::process::Output),
    Exit,
}

struct InputHandler<T: TermCursor + Write> {
    screen: Screen<T>,
}

impl<T: TermCursor + Write> InputHandler<T> {
    fn new(screen: Screen<T>) -> Self {
        InputHandler { screen }
    }

    pub fn input_from_prompt(
        &mut self,
        prompt: Option<String>,
        stdin: impl Iterator<Item = Result<Key, std::io::Error>>,
    ) -> Result<Input, InputError> {
        match prompt {
            Some(_) => {
                self.screen.show_prompt(&prompt.unwrap());
                self.screen.show_cursor();

                let input = input::input_from_keys(stdin, &mut self.screen.stdout)?;

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

                // To-do: `command should` return a result.
                // To-do: The cursor is shown previously in prompt_input.
                let mut command = CmdRunner::new(keymap.command.clone(), Some(i));
                // let stdout_mutex = Arc::new(Mutex::new(Some(stdout)));
                let output = command.run().unwrap();

                Ok(Process::Output(output))
            }
            Ok(Input::None) => {
                self.screen.show_cursor();
                drop(self.screen.stdout);

                let mut command = CmdRunner::new(keymap.command.clone(), None);
                let output = command.run().unwrap();

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

fn main() {
    let stdout = RawStdout::new().unwrap();
    let mut screen = Screen::new(stdout);

    screen.stdout.flush().unwrap();

    let keymaps = vec![
        Keymap::new('t', "Sleep", "sleep 2 && echo test && sleep 2"),
        Keymap::with_prompt(
            'c',
            "Git add and commit",
            "git add . && git commit -m \"{}\"",
            "Enter commit message:",
        ),
        Keymap::new('o', "Open script", "vi script.txt"),
        Keymap::new('s', "Run script.sh", "./script.sh"),
        Keymap::new('r', "cargo run --release", "cargon run --release"),
        Keymap::new(
            'a',
            "Run all",
            "/home/alex/bash/crop/script.sh &&
            /home/alex/rust/visual-center/target/release/visual_center &&
            /home/alex/bash/delete/script.sh",
        ),
    ];

    let menu_items: Vec<String> = keymaps
        .iter()
        .map(|keymap| format!("{}  {}", keymap.key, keymap.description))
        .collect();

    screen.clear_all();
    screen.show_prompt("Please select a command:");
    screen.show_menu(&menu_items);
    // screen.stdout.flush().unwrap();

    for key in stdin().keys() {
        match key.unwrap() {
            Key::Char('q') => {
                screen.show_cursor();
                break;
            }
            Key::Char(key) => {
                if let Some(keymap) = keymaps.iter().find(|k| k.key == key) {
                    let mut input_handler = InputHandler::new(screen);
                    let input =
                        input_handler.input_from_prompt(keymap.prompt.clone(), stdin().keys());

                    input_handler.process_input(input, keymap).unwrap();
                    break;
                }
            }
            _ => {}
        }
    }
}

// #[cfg(test)]
mod tests;
