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

    fn handle_quit(&mut self) {
        self.stdout
            .write_term(format_args!("{}", termion::cursor::Show))
            .unwrap();
    }

    fn prompt_input(&mut self, message: &str) {
        self.stdout
            .write_term(format_args!("{}{}", termion::cursor::Show, message))
            .unwrap();
        self.stdout.flush().unwrap();
    }

    fn handle_input_result(mut self, result: Result<Input, InputError>, keymap: &Keymap) {
        match result {
            Ok(Input::Text(i)) => {
                // Because the input doesn't start a newline
                self.stdout.write_term(format_args!("\r\n")).unwrap();

                self.stdout
                    .write_term(format_args!("{}", termion::cursor::Show))
                    .unwrap();

                self.stdout.flush().unwrap();

                drop(self.stdout);

                // To-do: `command should` return a result.
                // To-do: The cursor is shown previously in prompt_input.
                let mut command = CmdRunner::new(keymap.command, Some(&i));
                // let stdout_mutex = Arc::new(Mutex::new(Some(stdout)));

                command.run().unwrap();
            }
            Ok(Input::None) => {
                self.stdout
                    .write_term(format_args!("{}", termion::cursor::Show))
                    .unwrap();

                self.stdout.flush().unwrap();
                drop(self.stdout);

                let mut command = CmdRunner::new(keymap.command, None);

                command.run().unwrap();
            }
            Ok(Input::Exit) => {
                self.stdout.write_term(format_args!("\r\n")).unwrap();
            }
            Err(e) => {
                self.stdout
                    .write_term(format_args!("Invalid input: {}\r\n", e))
                    .unwrap();
            }
        };
    }

    fn handle_input(
        mut self,
        key: char,
        keymaps: &[Keymap],
        stdin: impl Iterator<Item = Result<Key, std::io::Error>>,
    ) {
        if let Some(keymap) = keymaps.iter().find(|k| k.key == key) {
            match keymap.prompt {
                Some(_) => {
                    self.prompt_input(keymap.prompt.unwrap());

                    let input = keymap::input::get_input(stdin, &mut self.stdout);

                    self.handle_input_result(input, &keymap);
                }
                None => {
                    self.handle_input_result(Ok(Input::None), &keymap);
                }
            }
        }
    }

    fn show_keymap_menu(&mut self, keymaps: &[Keymap]) {
        self.stdout
            .write_term(format_args!(
                "{}{}Please select a command:{}\r\n",
                termion::clear::All,
                termion::cursor::Goto(1, 1),
                termion::cursor::Hide,
            ))
            .unwrap();

        for keymap in keymaps {
            self.stdout
                .write_term(format_args!("{}  {}\r\n", keymap.key, keymap.description))
                .unwrap();
        }

        self.stdout.flush().unwrap();
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
            "Enter commit message\r\n",
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

    screen.show_keymap_menu(&keymaps);
    screen.stdout.flush().unwrap();

    for key in stdin().keys() {
        match key.unwrap() {
            Key::Char('q') => {
                screen.handle_quit();
                break;
            }
            Key::Char(c) => {
                screen.handle_input(c, &keymaps, stdin().keys());
                break;
            }
            _ => {}
        }
    }
}

// #[cfg(test)]
mod tests;
