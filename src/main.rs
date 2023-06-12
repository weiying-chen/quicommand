use keymap::cmd_runner::CmdRunner;
use keymap::input::{Input, InputError};
use keymap::keymap::Keymap;
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

struct Screen<T: TermCursor + Write> {
    stdout: T,
}

impl<T: TermCursor + Write> Screen<T> {
    //To-do: maybe functions like these should belong to `TermWriter`?
    fn new(stdout: T) -> Self {
        Screen { stdout }
    }

    fn add_newline(&mut self) {
        self.stdout.write_term(format_args!("\r\n")).unwrap();
    }

    fn show_cursor(&mut self) {
        self.stdout
            .write_term(format_args!("{}", termion::cursor::Show))
            .unwrap();
        self.stdout.flush().unwrap();
    }

    fn show_prompt(&mut self, message: &str) {
        self.stdout
            .write_term(format_args!("{}\r\n", message))
            .unwrap();
    }

    fn show_menu(&mut self, items: &[String]) {
        for item in items {
            self.stdout
                .write_term(format_args!("{}\r\n", item))
                .unwrap();
        }
    }

    fn clear_all(&mut self) {
        self.stdout
            .write_term(format_args!(
                "{}{}{}",
                termion::clear::All,
                termion::cursor::Goto(1, 1),
                termion::cursor::Hide,
            ))
            .unwrap();
    }

    fn input_from_prompt(
        &mut self,
        keymap: &Keymap,
        stdin: impl Iterator<Item = Result<Key, std::io::Error>>,
    ) -> Result<Input, InputError> {
        match keymap.prompt {
            Some(_) => {
                self.show_prompt(&keymap.prompt.clone().unwrap());
                self.show_cursor();

                let input = keymap::input::input_from_keys(stdin, &mut self.stdout)?;

                Ok(input)
            }
            None => Ok(Input::None),
        }
    }

    fn handle_input_result(mut self, result: Result<Input, InputError>, keymap: &Keymap) {
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
                    let input = screen.input_from_prompt(keymap, stdin().keys());

                    screen.handle_input_result(input, keymap);
                    break;
                }
            }
            _ => {}
        }
    }
}

// #[cfg(test)]
mod tests;
