// use std::io::Write;

pub struct Keymap {
    pub key: char,
    pub description: &'static str,
    pub command: &'static str,
    pub input_placeholder: &'static str,
}

//  `get_input` should be extracted out of `execute_command()`.
// impl Keymap {
//     pub fn execute_command(&mut self, input: String, stdout: &mut impl Write) {
//         self.command.execute(&input, stdout);
//     }
// }

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
