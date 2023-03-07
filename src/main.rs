use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

struct Command {
    key: char,
    description: &'static str,
    execute: fn(),
}

fn main() {
    // Turn on raw mode for the terminal
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Define the available commands
    let commands = vec![
        Command {
            key: 'f',
            description: "Feat",
            execute: || println!("Executing Feat command\r\n"),
        },
        Command {
            key: 'x',
            description: "Fix",
            execute: || println!("Executing Fix command\r\n"),
        },
    ];

    // Print the messages to the terminal
    write!(stdout, "Please select a command:\r\n").unwrap();
    for command in &commands {
        write!(stdout, "{}  {}\r\n", command.key, command.description).unwrap();
    }
    stdout.flush().unwrap();

    // Read input from stdin
    let input = stdin().keys();

    // Loop over each key that is pressed
    for key in input {
        // Match the key against the available commands
        match key.unwrap() {
            // If the key matches a command, execute it and break the loop
            Key::Char(k) if commands.iter().any(|c| c.key == k) => {
                let command = commands.iter().find(|c| c.key == k).unwrap();
                (command.execute)();
                break;
            }
            // If the key is any other printable character, print it to the screen
            Key::Char(c) => {
                write!(stdout, "You pressed: {}\r\n", c).unwrap();
                stdout.flush().unwrap();
            }
            // If the key is anything else, ignore it
            _ => {}
        }
    }
}
