use std::io::{stdin, stdout, Read, Write};
use termion::event::Key;
use termion::raw::IntoRawMode;

fn main() {
    // Turn on raw mode for the terminal
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Print the message to the terminal
    write!(stdout, "Press any key to continue...\r\n").unwrap();
    stdout.flush().unwrap();

    // Read a single byte from stdin
    let mut input = stdin().bytes();
    let key = input.next().unwrap().unwrap();

    // Print the key that was pressed
    write!(stdout, "You pressed: {:?}\r\n", Key::Char(key as char)).unwrap();
}
