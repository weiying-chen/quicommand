use std::fmt;
use std::io::{stdin, stdout, Write};
use std::process::Command;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

struct KeyboardShortcut {
    key: char,
    description: &'static str,
    command: &'static str,
    input_placeholder: &'static str,
}

enum InputError {
    NotUTF8(Vec<u8>),
    EmptyString,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputError::NotUTF8(bytes) => write!(
                f,
                "Input contained non-UTF8 bytes: {:?}",
                bytes
                    .iter()
                    .map(|b| format!("0x{:X}", b))
                    .collect::<Vec<_>>()
            ),
            InputError::EmptyString => write!(f, "Input was empty."),
        }
    }
}

impl KeyboardShortcut {
    fn execute_command(&self, stdout: &mut impl Write) {
        write!(stdout, "Enter commit message: ").unwrap();
        stdout.flush().unwrap();

        let input = match get_input(stdout) {
            Ok(i) => i,
            Err(e) => {
                write!(stdout, "\n\rInvalid input: {}\n\r", e).unwrap();
                return;
            }
        };

        let command = self.command.replace(self.input_placeholder, &input);

        // This combination makes commands print colors.
        let output = Command::new("script")
            .arg("-qec")
            .arg(command)
            .arg("/dev/null")
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    let output_str = String::from_utf8_lossy(&output.stdout);

                    for line in output_str.lines() {
                        write!(stdout, "{}\n\r", line).unwrap();
                    }
                } else {
                    let stderr_str = String::from_utf8_lossy(&output.stderr);
                    write!(
                        stdout,
                        "Command execution failed: {}\n\r",
                        stderr_str.trim()
                    )
                    .unwrap();
                }
            }
            Err(e) => {
                write!(stdout, "Error executing command: {:?}\r\n", e).unwrap();
            }
        }
    }
}

fn get_input(stdout: &mut impl Write) -> Result<String, InputError> {
    let mut input = String::new();

    for key in stdin().keys() {
        match key.unwrap() {
            // This is Enter.
            Key::Char('\n') => {
                if input.is_empty() {
                    return Err(InputError::EmptyString);
                } else {
                    break;
                }
            }
            Key::Char(c) => {
                let bytes = vec![c as u8];
                match std::str::from_utf8(&bytes) {
                    Ok(_) => {
                        input.push(c);
                        write!(stdout, "{}", c).unwrap();
                    }
                    Err(_) => {
                        return Err(InputError::NotUTF8(bytes));
                    }
                }
            }
            Key::Esc => {
                return Err(InputError::NotUTF8(vec![0x1b]));
            }
            Key::Backspace => {
                // To prevent deleting "Enter commit message:"
                if input.is_empty() {
                    continue;
                }

                input.pop();

                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Left(1),
                    termion::clear::UntilNewline,
                )
                .unwrap();
            }
            _ => {}
        }

        stdout.flush().unwrap();
    }

    let input = input.trim().to_owned();

    Ok(input)
}

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}Please select a command:\r\n{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide,
    )
    .unwrap();

    let keyboard_shortcuts = vec![
        KeyboardShortcut {
            key: 'f',
            description: "Feat: adds a new feature to the product",
            command: "git add . && git commit -m 'Feat: {}'",
            input_placeholder: "{}",
        },
        KeyboardShortcut {
            key: 'x',
            description: "Fix: fixes a defect in a feature",
            command: "git add . && git commit -m 'Fix: {}'",
            input_placeholder: "{}",
        },
        KeyboardShortcut {
            key: 'r',
            description: "Refac: changes a feature's code but not its behavior",
            command: "git add . && git commit -m 'Chore: {}'",
            input_placeholder: "{}",
        },
        KeyboardShortcut {
            key: 'c',
            description: "Chore: changes that are not related any feature",
            // command: "git add . && git commit -m 'Chore: {}'",
            command: "git status",
            input_placeholder: "{}",
        },
    ];

    for keyboard_shortcut in &keyboard_shortcuts {
        write!(
            stdout,
            "{}  {}\r\n",
            keyboard_shortcut.key, keyboard_shortcut.description
        )
        .unwrap();
    }

    stdout.flush().unwrap();

    for key in stdin().keys() {
        match key.unwrap() {
            // TODO: Esc should perform the same action.
            Key::Char('q') => {
                write!(stdout, "{}", termion::cursor::Show).unwrap();
                break;
            }
            Key::Char(c) if keyboard_shortcuts.iter().any(|k| k.key == c) => {
                let keyboard_shortcut = keyboard_shortcuts.iter().find(|k| k.key == c).unwrap();

                // Raw mode has to be suspented to collect input.
                // stdout.suspend_raw_mode().unwrap();
                write!(stdout, "{}", termion::cursor::Show).unwrap();
                keyboard_shortcut.execute_command(&mut stdout);
                // stdout.activate_raw_mode().unwrap();
                break;
            }
            _ => {}
        }
    }
}
