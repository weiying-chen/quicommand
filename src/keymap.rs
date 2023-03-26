use crate::input::{Input, InputError};
use std::{io::Write, process::Command};

pub struct Keymap {
    pub key: char,
    pub description: &'static str,
    pub command: &'static str,
    pub input_placeholder: &'static str,
}

//  `get_input` should be extracted out of `execute_command()`.
impl Keymap {
    pub fn execute_command(&self, input: String, stdout: &mut impl Write) {
        // This combination makes commands print colors.

        let command = self.command.replace(self.input_placeholder, &input);

        let output = Command::new("script")
            .arg("-qec")
            .arg(command)
            .arg("/dev/null")
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    let stdout_str = String::from_utf8_lossy(&output.stdout);

                    // This places the output on a new line.
                    write!(stdout, "\r\n").unwrap();

                    for line in stdout_str.lines() {
                        write!(stdout, "{}\r\n", line).unwrap();
                    }
                } else {
                    let stdout_str = String::from_utf8_lossy(&output.stdout);
                    let stderr_str = String::from_utf8_lossy(&output.stderr);

                    // This places the output on a new line.
                    write!(stdout, "\r\n").unwrap();

                    write!(stdout, "Standard error: {}\r\n", stderr_str.trim()).unwrap();
                    write!(stdout, "Standard output: {}\r\n", stdout_str.trim()).unwrap();
                }
            }
            Err(e) => {
                write!(stdout, "Error executing command: {:?}\r\n", e).unwrap();
            }
        }
    }
}

//TODO: test Left, Right, Esc, and Backspace.
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::io::stdout;
//     use termion::raw::IntoRawMode;

//     #[test]
//     fn test_get_input() {
//         let keys = vec![
//             Ok(Key::Char('h')),
//             Ok(Key::Char('e')),
//             Ok(Key::Char('l')),
//             Ok(Key::Char('l')),
//             Ok(Key::Char('o')),
//             Ok(Key::Char('\n')),
//         ];

// let mut stdout = stdout().into_raw_mode().unwrap();
// let result = get_input(keys.into_iter(), &mut stdout);

// assert_eq!(result.unwrap(), Input::Text(String::from("hello")));
//     }
// }
