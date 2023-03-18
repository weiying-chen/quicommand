use cc::input::{Input, InputError};
use cc::key_handler::KeyHandler;
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

impl KeyboardShortcut {
    fn execute_command(&self, stdout: &mut impl Write) {
        write!(stdout, "Enter commit message: ").unwrap();
        stdout.flush().unwrap();

        let input = match get_input(stdout) {
            Ok(Input::Text(i)) => i,
            Ok(Input::Exit) => {
                write!(stdout, "\r\n").unwrap();
                return;
            }
            Err(e) => {
                write!(stdout, "\r\nInvalid input: {}\r\n", e).unwrap();
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
                        write!(stdout, "{}\r\n", line).unwrap();
                    }
                } else {
                    let stderr_str = String::from_utf8_lossy(&output.stderr);

                    write!(
                        stdout,
                        "Command execution failed: {}\r\n",
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

fn get_input(stdout: &mut impl Write) -> Result<Input, InputError> {
    let input = String::new();
    // let mut y = 1;
    let mut key_handler = KeyHandler::new(input);

    for key in stdin().keys() {
        match key.unwrap() {
            Key::Char('\n') => return key_handler.enter(),
            Key::Esc => return Ok(Input::Exit),
            Key::Char(c) => key_handler.char(stdout, c)?,
            Key::Left => key_handler.left(stdout),
            Key::Right => key_handler.right(stdout),
            Key::Backspace => key_handler.backspace(stdout),
            _ => {}
        }

        stdout.flush().unwrap();
    }

    // This places the output on a new line.
    write!(stdout, "\r\n").unwrap();

    // TODO: maybe a function should return the input instead?
    let input = key_handler.input.trim().to_owned();

    Ok(Input::Text(input))
}

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}Please select a command:{}\r\n",
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
            command: "git add . && git commit -m 'Refac: {}'",
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
