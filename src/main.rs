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

//To-do: maybe functions like these should belong to `TermWriter`?
fn handle_quit(stdout: &mut RawStdout) {
    stdout
        .write_term(format_args!("{}", termion::cursor::Show))
        .unwrap();
}

fn prompt_input<T: TermCursor + Write>(message: &str, stdout: &mut T) {
    stdout
        .write_term(format_args!("{}{}", termion::cursor::Show, message))
        .unwrap();
    stdout.flush().unwrap();
}

fn handle_input_result<T: TermCursor + Write>(
    result: Result<Input, InputError>,
    keymap: &Keymap,
    mut stdout: T,
) {
    match result {
        Ok(Input::Text(i)) => {
            // Because the input doesn't start a newline
            stdout.write_term(format_args!("\r\n")).unwrap();

            stdout
                .write_term(format_args!("{}", termion::cursor::Show))
                .unwrap();

            stdout.flush().unwrap();

            drop(stdout);

            // To-do: `command should` return a result.
            // To-do: The cursor is shown previously in prompt_input.
            let mut command = CmdRunner::new(keymap.command, Some(&i));
            // let stdout_mutex = Arc::new(Mutex::new(Some(stdout)));

            command.run().unwrap();
        }
        Ok(Input::None) => {
            stdout
                .write_term(format_args!("{}", termion::cursor::Show))
                .unwrap();

            stdout.flush().unwrap();

            drop(stdout);

            let mut command = CmdRunner::new(keymap.command, None);
            // let stdout_mutex = Arc::new(Mutex::new(Some(stdout)));

            // command.run(stdout_mutex);

            command.run().unwrap();
            // println!("OUTPUT: {:?}", &output);
        }
        Ok(Input::Exit) => {
            stdout.write_term(format_args!("\r\n")).unwrap();
        }
        Err(e) => {
            stdout
                .write_term(format_args!("Invalid input: {}\r\n", e))
                .unwrap();
        }
    };
}

fn handle_input<T: TermCursor + Write>(
    key: char,
    keymaps: &[Keymap],
    stdin: impl Iterator<Item = Result<Key, std::io::Error>>,
    mut stdout: T,
) {
    if let Some(keymap) = keymaps.iter().find(|k| k.key == key) {
        if (keymap.command).contains("{}") {
            // if let Some(prompt_str) = &keymap.prompt {
            // To-do: if there is a `{}`, a prompt should be required.
            prompt_input(keymap.prompt.unwrap(), &mut stdout);
            // }

            let input = keymap::input::get_input(stdin, &mut stdout);

            handle_input_result(input, &keymap, stdout);
        } else {
            handle_input_result(Ok(Input::None), &keymap, stdout);
        }
    }
}

fn show_keymap_menu<T: TermCursor + Write>(keymaps: &[Keymap], stdout: &mut T) {
    stdout
        .write_term(format_args!(
            "{}{}Please select a command:{}\r\n",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide,
        ))
        .unwrap();

    for keymap in keymaps {
        stdout
            .write_term(format_args!("{}  {}\r\n", keymap.key, keymap.description))
            .unwrap();
    }

    stdout.flush().unwrap();
}

fn main() {
    let mut stdout = RawStdout::new().unwrap();

    stdout.flush().unwrap();

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

    show_keymap_menu(&keymaps, &mut stdout);
    stdout.flush().unwrap();

    for key in stdin().keys() {
        match key.unwrap() {
            Key::Char('q') => {
                handle_quit(&mut stdout);
                break;
            }
            Key::Char(c) => {
                handle_input(c, &keymaps, stdin().keys(), stdout);
                break;
            }
            _ => {}
        }
    }
}

// #[cfg(test)]
mod tests;
