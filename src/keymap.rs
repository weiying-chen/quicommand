// use std::io::Write;

pub struct Keymap {
    pub key: char,
    pub description: &'static str,
    pub command: &'static str,
    // pub input_placeholder: &'static str,
}

//  `get_input` should be extracted out of `execute_command()`.
// impl Keymap {
//     pub fn execute_command(&mut self, input: String, stdout: &mut impl Write) {
//         self.command.execute(&input, stdout);
//     }
// }
