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
        stdin().read_line(&mut message).unwrap();

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
                    write!(
                        stdout,
                        "ERR: {}\r\n",
                        String::from_utf8_lossy(&output.stderr)
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

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    let keyboard_shortcuts = vec![
        KeyboardShortcut {
            key: 'f',
            description: "Feat: adds a new feature to the application",
            command: "git add . && git commit -m 'Feat: {}'",
            // command: "git -c color.status=always status",
            // command: "ls --color=always",
            message_placeholder: "{}",
        },
        KeyboardShortcut {
            key: 'x',
            description: "Fix: fixes a defect in the application",
            command: "git add . && git commit -m 'Fix: {}'",
            // command: "git -c color.status=always status",
            // command: "ls --color=always",
            message_placeholder: "{}",
        },
    ];

    write!(
        stdout,
        "{}{}Please select a command:\r\n{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide,
    )
    .unwrap();

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
            Key::Char(k) if keyboard_shortcuts.iter().any(|s| s.key == k) => {
                let keyboard_shortcut = keyboard_shortcuts.iter().find(|c| c.key == k).unwrap();

                // Raw mode has to be suspented to collect input.
                stdout.suspend_raw_mode().unwrap();
                write!(stdout, "{}", termion::cursor::Show).unwrap();
                keyboard_shortcut.execute_command(&mut stdout);
                stdout.activate_raw_mode().unwrap();
                break;
            }
            Key::Char(c) => {
                write!(stdout, "You pressed: {}\r\n", c).unwrap();
                stdout.flush().unwrap();
            }
            _ => {}
        }
    }
}
