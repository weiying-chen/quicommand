use std::io::{stdin, stdout, Write};
use std::process::Command;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

struct KeyboardShortcut {
    key: char,
    description: &'static str,
    command: &'static str,
}

impl KeyboardShortcut {
    fn execute_command(&self, stdout: &mut impl Write) {
        let output = Command::new("sh").arg("-c").arg(self.command).status();

        match output {
            Ok(status) => {
                if status.success() {
                    write!(stdout, "Command executed successfully\r\n").unwrap();
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
    // Turn on raw mode for the terminal
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Define the available commands

    let keyboard_shortcuts = vec![
        KeyboardShortcut {
            key: 'c',
            description: "This is a custom command",
            command: "echo 'Custom command executed\r\n'",
        },
        KeyboardShortcut {
            key: 'g',
            description: "This is a custom command",
            command: "git status",
        },
    ];

    // Print the messages to the terminal
    write!(stdout, "Please select a command:\r\n").unwrap();

    for keyboard_shortcut in &keyboard_shortcuts {
        write!(
            stdout,
            "{}  {}\r\n",
            keyboard_shortcut.key, keyboard_shortcut.description
        )
        .unwrap();
    }

    stdout.flush().unwrap();

    // Read input from stdin
    let input = stdin().keys();

    // Loop over each key that is pressed
    for key in input {
        // Match the key against the available commands
        match key.unwrap() {
            // If the key matches a command, execute it and break the loop
            Key::Char(k) if keyboard_shortcuts.iter().any(|c| c.key == k) => {
                let keyboard_shortcut = keyboard_shortcuts.iter().find(|c| c.key == k).unwrap();
                keyboard_shortcut.execute_command(&mut stdout);
                break;
            }
            // Output other characters
            Key::Char(c) => {
                write!(stdout, "You pressed: {}\r\n", c).unwrap();
                stdout.flush().unwrap();
            }
            // If the key is anything else, ignore it
            _ => {}
        }
    }
}
