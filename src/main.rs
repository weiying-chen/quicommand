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

        let mut message = String::new();

        // stdin().read_line(&mut message).unwrap();

        for key in stdin().keys() {
            match key.unwrap() {
                Key::Char('\n') => break,
                Key::Char(c) => {
                    message.push(c);
                    write!(stdout, "{}", c).unwrap();
                }
                Key::Esc => {
                    // User pressed Esc, exit early.
                    write!(stdout, "\n\rCommand execution cancelled.\n\r").unwrap();
                    return;
                }

                // TODO: "Enter message:" can be deleted too.
                Key::Backspace => {
                    // Handle backspace to delete characters from the message.
                    message.pop();
                    write!(
                        stdout,
                        "{}{} {}",
                        termion::cursor::Left(1),
                        termion::clear::UntilNewline,
                        termion::cursor::Left(1),
                    )
                    .unwrap();
                }
                _ => {}
            }
            stdout.flush().unwrap();
        }

        // Remove new line character.
        let message = message.trim();
        let command = self.command.replace(self.message_placeholder, message);
        let output = Command::new("sh").arg("-c").arg(command).output();

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
            description: "Feat: adds a new feature to the application",
            command: "git add . && git commit -m 'Feat: {}'",
            // command: "git -c color.status=always status",
            // command: "ls --color=always",
            // This placeholder later can be customized.
            message_placeholder: "{}",
        },
        KeyboardShortcut {
            key: 'x',
            description: "Fix: fixes a defect in the application",
            // command: "git add . && git commit -m 'Fix: {}'",
            // command: "git -c color.status=always status",
            command: "ls --color=always",
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
