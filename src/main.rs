use std::io::{stdin, stdout, Write};
use std::process::Command;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

struct KeyboardShortcut {
    key: char,
    description: &'static str,
    command: &'static str,
    message_placeholder: &'static str,
}

impl KeyboardShortcut {
    fn execute_command(&self, stdout: &mut impl Write) {
        write!(stdout, "Enter message: ").unwrap();
        stdout.flush().unwrap();

        let message = read_message_from_user(stdout);

        match message {
            Some(m) => {
                let command = self.command.replace(self.message_placeholder, &m);

                println!("{}", m);

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
                            write!(stdout, "Command execution failed\r\n").unwrap();
                        }
                    }
                    Err(e) => {
                        write!(stdout, "Error executing command: {:?}\r\n", e).unwrap();
                    }
                }
            }
            None => {
                write!(stdout, "\n\rCommand execution cancelled.\n\r").unwrap();
                return;
            }
        }
    }
}

fn read_message_from_user(stdout: &mut impl Write) -> Option<String> {
    let mut message = String::new();

    for key in stdin().keys() {
        match key.unwrap() {
            Key::Char('\n') => break,
            Key::Char(c) => {
                message.push(c);
                write!(stdout, "{}", c).unwrap();
            }
            Key::Esc => {
                return None;
            }

            Key::Backspace => {
                if message.is_empty() {
                    continue;
                }

                message.pop();
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

    let message = message.trim().to_owned();

    Some(message)
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
            message_placeholder: "{}",
        },
        KeyboardShortcut {
            key: 'x',
            description: "Fix: fixes a defect in a feature",
            command: "git add . && git commit -m 'Fix: {}'",
            message_placeholder: "{}",
        },
        KeyboardShortcut {
            key: 'r',
            description: "Refac: changes a feature's code but not its behavior",
            command: "git add . && git commit -m 'Chore: {}'",
            message_placeholder: "{}",
        },
        KeyboardShortcut {
            key: 'c',
            description: "Chore: changes that are not related any feature",
            // command: "git add . && git commit -m 'Chore: {}'",
            command: "git status",
            message_placeholder: "{}",
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
