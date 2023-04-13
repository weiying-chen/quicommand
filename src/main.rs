use command_launcher::cmd_runner::CmdRunner;
use command_launcher::input::{Input, InputError};
use command_launcher::keymap::Keymap;
use command_launcher::term_writer::TermCursor;
use std::io::{stdin, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

//To-do: move this into its own file.

struct CustomRawTerminal {
    buffer: RawTerminal<std::io::Stdout>,
}

impl CustomRawTerminal {
    pub fn new() -> std::io::Result<Self> {
        let buffer = std::io::stdout().into_raw_mode()?;
        Ok(Self { buffer })
    }
}

impl Write for CustomRawTerminal {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buffer.flush()
    }
}

impl TermCursor for CustomRawTerminal {
    fn write_term(&mut self, fmt: std::fmt::Arguments) -> std::io::Result<()> {
        std::io::Write::write_fmt(self, fmt)
    }

    fn cursor_position(&mut self) -> Result<(u16, u16), std::io::Error> {
        termion::cursor::DetectCursorPos::cursor_pos(self)
    }
}

//To-do: maybe functions like these should belong to `TermWriter`?
fn handle_quit<T: TermCursor>(stdout: &mut T) {
    stdout
        .write_term(format_args!("{}", termion::cursor::Show))
        .unwrap();
}

fn prompt_input<T: TermCursor + Write>(message: &str, stdout: &mut T) {
    stdout
        .write_term(format_args!("{}{}:", termion::cursor::Show, message))
        .unwrap();
    stdout.flush().unwrap();
}

// To-do: this function is doing too many things at the same time.

fn handle_input_result<T: TermCursor + Write>(
    result: Result<Input, InputError>,
    keymap: &Keymap,
    stdout: &mut T,
) {
    match result {
        Ok(Input::Text(i)) => {
            // To-do `command should` return a result.
            let mut command = CmdRunner::new(keymap.command, &i);
            command.execute(stdout);
        }
        Ok(Input::Exit) => {
            stdout.write_term(format_args!("\r\n")).unwrap();
        }
        Err(e) => {
            stdout
                .write_term(format_args!("\r\nInvalid input: {}\r\n", e))
                .unwrap();
            stdout.write_term(format_args!("\r\n")).unwrap();
        }
    };
}

fn handle_input<T: TermCursor + Write>(
    key: char,
    keymaps: &[Keymap],
    stdin: impl Iterator<Item = Result<Key, std::io::Error>>,
    stdout: &mut T,
) {
    if let Some(keymap) = keymaps.iter().find(|k| k.key == key) {
        //To-do: change tests according to this.
        let message = "Enter commit message";

        prompt_input(message, stdout);

        let input = command_launcher::input::get_input(stdin, stdout);

        handle_input_result(input, &keymap, stdout);
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
    // let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdout = CustomRawTerminal::new().unwrap();

    let keymaps = vec![Keymap {
        key: 's',
        description: "Run echo",
        command: "echo {}",
        // input_placeholder: "{}",
    }];

    show_keymap_menu(&keymaps, &mut stdout);

    stdout.flush().unwrap();

    // let mut custom_stdout = Stdout {};

    for key in stdin().keys() {
        match key.unwrap() {
            Key::Char('q') => {
                handle_quit(&mut stdout);
                break;
            }
            Key::Char(c) => {
                handle_input(c, &keymaps, stdin().keys(), &mut stdout);
                break;
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Stdout

    #[derive(Debug)]
    struct Stdout {
        buffer: Vec<u8>,
        pos: (u16, u16),
    }

    impl Stdout {
        pub fn new() -> Self {
            let buffer = Vec::new();
            Self {
                buffer,
                pos: (1, 1),
            }
        }
    }

    impl Write for Stdout {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.buffer.write(buf)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.buffer.flush()
        }
    }

    impl TermCursor for Stdout {
        fn write_term(&mut self, fmt: std::fmt::Arguments) -> std::io::Result<()> {
            const INPUT_START: &str = "\u{1b}[2K";

            if fmt.to_string().contains(INPUT_START) {
                self.pos.0 += 1;
            }

            // To-do: maybe shouldn't be using `unwrap()` here.
            std::io::Write::write_fmt(self, fmt).unwrap();

            Ok(())
        }

        fn cursor_position(&mut self) -> Result<(u16, u16), std::io::Error> {
            Ok(self.pos)
        }
    }

    fn get_keymaps() -> Vec<Keymap> {
        vec![Keymap {
            key: 't',
            description: "Test keymap",
            command: "echo {}",
        }]
    }

    #[test]
    fn test_show_keymap_menu() {
        let keymaps = get_keymaps();
        let mut stdout = Stdout::new();

        show_keymap_menu(&keymaps, &mut stdout);

        let stdout_str = String::from_utf8(stdout.buffer).unwrap();

        assert!(stdout_str.contains("Please select a command"));
        assert!(stdout_str.contains("t  Test keymap"));
    }

    #[test]
    fn test_show_input_instruction() {
        const PROMPT_MESSAGE: &str = "Enter commit message:";
        let mut stdout = Stdout::new();

        prompt_input(PROMPT_MESSAGE, &mut stdout);

        let stdout_str = String::from_utf8(stdout.buffer).unwrap();

        assert!(stdout_str.contains(PROMPT_MESSAGE));
    }

    #[test]
    fn test_empty_input() {
        // To-do: `mock_stdin` isn't being used for the purpose of this test
        let stdin = vec![Ok(Key::Char('\n'))].into_iter();
        let mut stdout = Stdout::new();
        let input = command_launcher::input::get_input(stdin, &mut stdout);
        let keymaps = get_keymaps();

        handle_input_result(input, &keymaps[0], &mut stdout);

        let stdout_str = String::from_utf8(stdout.buffer).unwrap();

        assert!(stdout_str.contains("Input was empty"));
    }

    #[test]
    fn test_exit() {
        let stdin = vec![Ok(Key::Esc)].into_iter();
        let mut stdout = Stdout::new();
        let input = command_launcher::input::get_input(stdin, &mut stdout);
        let keymaps = get_keymaps();

        handle_input_result(input, &keymaps[0], &mut stdout);

        let stdout_str = String::from_utf8(stdout.buffer).unwrap();

        assert!(stdout_str.contains("\r\n"));
    }

    #[test]
    fn test_run_command() {
        let stdin = vec![
            Ok(Key::Char('a')),
            Ok(Key::Char('b')),
            Ok(Key::Char('c')),
            Ok(Key::Char('\n')),
        ];

        let mut stdout = Stdout::new();
        let input = command_launcher::input::get_input(stdin.into_iter(), &mut stdout);
        let keymaps = get_keymaps();

        handle_input_result(input, &keymaps[0], &mut stdout);

        let stdout_str = String::from_utf8(stdout.buffer).unwrap();

        assert!(stdout_str.contains("abc\r\n"));
    }
}
