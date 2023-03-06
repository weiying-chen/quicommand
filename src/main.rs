use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    // Turn on raw mode for the terminal
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Print the message to the terminal
    write!(stdout, "Press any key to continue...\r\n").unwrap();
    stdout.flush().unwrap();

    // Read input from stdin
    let input = stdin().keys();

    // Loop over each key that is pressed
    for key in input {
        // Match the key against different patterns
        match key.unwrap() {
            // If the key is the "q" key, exit the program
            Key::Char('q') => {
                write!(stdout, "Exiting...\r\n").unwrap();
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
