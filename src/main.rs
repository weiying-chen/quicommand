use command_launcher::cmd_runner::CmdRunner;
use command_launcher::input::Input;
use command_launcher::keymap::Keymap;
use command_launcher::term_writer::CursorPos;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

struct CustomRawTerminal {
    stdout: RawTerminal<std::io::Stdout>,
}

impl CustomRawTerminal {
    pub fn new() -> std::io::Result<Self> {
        let stdout = std::io::stdout().into_raw_mode()?;
        Ok(Self { stdout })
    }
}

impl Write for CustomRawTerminal {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stdout.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stdout.flush()
    }
}

impl CursorPos for CustomRawTerminal {
    fn write_term(&mut self, fmt: std::fmt::Arguments) -> std::io::Result<()> {
        std::io::Write::write_fmt(self, fmt)
    }

    fn cursor_position(&self) -> Result<(u16, u16), std::io::Error> {
        Ok((2, 2))
    }
}

fn handle_quit(mut stdout: impl Write) {
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

// To-do: make this function more reusable.
fn handle_command<T: CursorPos + Write>(key: char, keymaps: &[Keymap], stdout: &mut T) {
    if let Some(keymap) = keymaps.iter().find(|k| k.key == key) {
        // write!(stdout, "{}Enter commit message: ", termion::cursor::Show).unwrap();
        stdout
            .write_term(format_args!(
                "{}Enter commit message:",
                termion::cursor::Show
            ))
            .unwrap();

        stdout.flush().unwrap();

        let input = command_launcher::input::get_input(stdin().keys(), stdout);

        match input {
            Ok(Input::Text(i)) => {
                let mut command = CmdRunner::new(keymap.command, &i);
                // To-do: the command should return a result?
                command.execute(stdout);
            }
            Ok(Input::Exit) => {
                // write!(stdout, "\r\n").unwrap();
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
}

fn main() {
    // let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdout = CustomRawTerminal::new().unwrap();

    write!(
        stdout,
        "{}{}Please select a command:{}\r\n",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide,
    )
    .unwrap();

    let keymaps = vec![
        Keymap {
            key: 'f',
            description: "Feat: adds a new feature to the product",
            command: "git add . && git commit -m 'Feat: {}'",
            // input_placeholder: "{}",
        },
        Keymap {
            key: 'x',
            description: "Fix: fixes a defect in a feature",
            command: "git add . && git commit -m 'Fix: {}'",
            // input_placeholder: "{}",
        },
        Keymap {
            key: 'r',
            description: "Refac: changes a feature's code but not its behavior",
            command: "git add . && git commit -m 'Refac: {}'",
            // input_placeholder: "{}",
        },
        Keymap {
            key: 'd',
            description: "Docs: changes related to documentation",
            command: "git add . && git commit -m 'Docs: {}'",
            // input_placeholder: "{}",
        },
        Keymap {
            key: 's',
            description: "Run git status",
            command: "git status",
            // input_placeholder: "{}",
        },
    ];

    for keymap in &keymaps {
        write!(stdout, "{}  {}\r\n", keymap.key, keymap.description).unwrap();
    }

    stdout.flush().unwrap();

    // let mut custom_stdout = Stdout {};

    for key in stdin().keys() {
        match key.unwrap() {
            Key::Char('q') => {
                handle_quit(&mut stdout);
                break;
            }
            Key::Char(c) => {
                handle_command(c, &keymaps, &mut stdout);
                break;
            }
            _ => {}
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::io::Cursor;

//     #[test]
//     fn test_handle_quit() {
//         let mut stdout = Cursor::new(Vec::new());
//         handle_quit(&mut stdout);
//         assert_eq!(stdout.into_inner(), b"\x1B[?25h"); // '\x1B[?25h' is the escape code for "show cursor"
//     }

//     #[test]
//     fn test_handle_command() {
//         let mut stdout = Cursor::new(Vec::new());
//         let keymaps = vec![Keymap {
//             key: 't',
//             description: "Test keymap",
//             command: "echo {}",
//             // input_placeholder: "{}",
//         }];
//         handle_command('t', &keymaps, &mut stdout);
//         assert_eq!(stdout.into_inner(), b"Enter commit message: ");

//         // Write test here
//     }
// }
