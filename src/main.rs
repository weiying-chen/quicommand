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

    fn cursor_position(&mut self) -> Result<(u16, u16), std::io::Error> {
        termion::cursor::DetectCursorPos::cursor_pos(self)
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

fn show_keymap_menu<T: CursorPos + Write>(keymaps: &[Keymap], stdout: &mut T) {
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
        description: "Run git status",
        command: "git status",
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
                handle_command(c, &keymaps, &mut stdout);
                break;
            }
            _ => {}
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use std::io::Cursor;

//     use super::*;

//     // Stdout

//     #[derive(Debug)]
//     struct Stdout {
//         stdout: Cursor<Vec<u8>>,
//     }

//     impl Stdout {
//         pub fn new() -> Self {
//             let stdout = Cursor::new(Vec::new());
//             Self { stdout }
//         }
//     }

//     impl CursorPos for Stdout {
//         fn write_term(&mut self, fmt: std::fmt::Arguments) -> std::io::Result<()> {
//             Ok(())
//         }

//         fn cursor_position(&mut self) -> Result<(u16, u16), std::io::Error> {
//             Ok((2, 2))
//         }
//     }

//     impl Write for Stdout {
//         fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//             std::io::Write::write(&mut std::io::stdout(), buf)
//         }

//         fn flush(&mut self) -> std::io::Result<()> {
//             std::io::Write::flush(&mut std::io::stdout())
//         }
//     }

//     #[test]
//     fn test_show_keymap_menu() {
//         let keymaps = vec![Keymap {
//             key: 't',
//             description: "Test keymap",
//             command: "echo {}",
//         }];

//         let mut stdout = Stdout::new();

//         show_keymap_menu(&keymaps, &mut stdout);

//         let stdout_str = String::from_utf8(stdout.stdout.into_inner()).unwrap();

//         println!("Output: {}", stdout_str);

//         // let stdout_str = String::from_utf8(stdout.to_string());

//         // print!("output: {}", stdout_str);
//         assert!(stdout_str.contains("Please select a command"));
//         // assert!(stdout_str.contains("t  Test keymap"));
//     }

//     #[test]
//     fn test_show_input_instruction() {
//         let keymaps = vec![Keymap {
//             key: 't',
//             description: "Test keymap",
//             command: "echo {}",
//         }];

//         // To-do: `mock_stdin` isn't being used for the purpose of this test
//         let mock_stdin = vec![Ok(Key::Char('t'))].into_iter();
//         let mut stdout = Stdout;

//         handle_command('t', &keymaps, &mut stdout);

//         let stdout_str = String::from_utf8(stdout.into_inner()).unwrap();

//         // println!("output: {}", stdout_str);
//         // println!("==");
//         assert!(stdout_str.contains("Enter commit message:"));
//     }
// }
