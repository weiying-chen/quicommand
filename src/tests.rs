// use super::*;

// // Stdout

// #[derive(Debug)]
// struct MockStdout {
//     buffer: Vec<u8>,
//     cursor_pos: (u16, u16),
// }

// impl MockStdout {
//     pub fn new() -> Self {
//         let buffer = Vec::new();

//         Self {
//             buffer,
//             cursor_pos: (1, 1),
//         }
//     }
// }

// impl Write for MockStdout {
//     fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//         self.buffer.write(buf)
//     }

//     fn flush(&mut self) -> std::io::Result<()> {
//         self.buffer.flush()
//     }
// }

// impl TermCursor for MockStdout {
//     fn write_term(&mut self, fmt: std::fmt::Arguments) -> std::io::Result<()> {
//         const INPUT_START: &str = "\u{1b}[2K";

//         if fmt.to_string().contains(INPUT_START) {
//             self.cursor_pos.0 += 1;
//         }

//         self.write_fmt(fmt).unwrap();

//         Ok(())
//     }

//     fn get_cursor_pos(&mut self) -> Result<(u16, u16), std::io::Error> {
//         Ok(self.cursor_pos)
//     }
// }

// fn get_keymaps<'a>() -> Vec<Keymap> {
//     vec![Keymap::new(
//         't',
//         "Test keymap",
//         "echo 'test'",
//     )]
// }

// fn get_keymaps_with_prompt<'a>() -> Vec<Keymap> {
//     vec![Keymap::new(
//         't',
//         "Test keymap",
//         "echo {}",
//     )]
// }

// #[test]
// fn test_show_keymap_menu() {
//     let keymaps = get_keymaps();
//     let mut stdout = MockStdout::new();

//     show_keymap_menu(&keymaps, &mut stdout);

//     let stdout_str = String::from_utf8(stdout.buffer).unwrap();

//     assert!(stdout_str.contains("Please select a command"));
//     assert!(stdout_str.contains("t  Test keymap"));
// }

// #[test]
// fn test_show_input_instruction() {
//     const PROMPT_MESSAGE: &str = "Enter commit message:";
//     let mut stdout = MockStdout::new();

//     prompt_input(PROMPT_MESSAGE, &mut stdout);

//     let stdout_str = String::from_utf8(stdout.buffer).unwrap();

//     assert!(stdout_str.contains(PROMPT_MESSAGE));
// }

// #[test]
// fn test_empty_input() {
//     let stdin = vec![Ok(Key::Char('\n'))].into_iter();
//     let mut stdout = MockStdout::new();
//     let input = keymap::input::get_input(stdin, &mut stdout);
//     let keymaps = get_keymaps();

//     handle_input_result(input, &keymaps[0], stdout);

//     let stdout_str = String::from_utf8(stdout.buffer).unwrap();

//     assert!(stdout_str.contains("Input was empty"));
// }

// #[test]
// fn test_exit() {
//     let stdin = vec![Ok(Key::Esc)].into_iter();
//     let mut stdout = MockStdout::new();
//     let input = keymap::input::get_input(stdin, &mut stdout);
//     let keymaps = get_keymaps();

//     handle_input_result(input, &keymaps[0], stdout);

//     let stdout_str = String::from_utf8(stdout.buffer).unwrap();

//     assert!(stdout_str.contains("\r\n"));
// }

// // #[test]
// // fn test_run_command() {
// //     let stdin = vec![
// //         Ok(Key::Char('a')),
// //         Ok(Key::Char('b')),
// //         Ok(Key::Char('c')),
// //         Ok(Key::Char('\n')),
// //     ];

// //     let mut stdout = MockStdout::new();
// //     let input = keymap::input::get_input(stdin.into_iter(), &mut stdout);
// //     let keymaps = get_keymaps();

// //     handle_input_result(input, &keymaps[0], &mut stdout);

// //     let stdout_str = String::from_utf8(stdout.buffer).unwrap();

// //     assert!(stdout_str.contains("abc"));
// // }

// #[test]
// fn test_run_command() {
//     let stdin = vec![
//         Ok(Key::Char('a')),
//         Ok(Key::Char('b')),
//         Ok(Key::Char('c')),
//         Ok(Key::Char('\n')),
//     ];

//     let mut stdout = MockStdout::new();
//     // let input = keymap::input::get_input(stdin.into_iter(), &mut stdout);
//     let keymaps = get_keymaps();

//     // handle_input_result(input, &keymaps[0], &mut stdout);
//     handle_input('t', &keymaps, stdin.into_iter(), stdout);

//     let stdout_str = String::from_utf8(stdout.buffer).unwrap();

//     println!("STDOUT_STR: {:?}", stdout_str);

//     assert!(stdout_str.contains("test"));
// }

// #[test]
// fn test_run_commmand_with_prompt() {
//     let stdin = vec![
//         Ok(Key::Char('a')),
//         Ok(Key::Char('b')),
//         Ok(Key::Char('c')),
//         Ok(Key::Char('\n')),
//     ];

//     let mut stdout = MockStdout::new();
//     let keymaps = get_keymaps_with_prompt();

//     handle_input('t', &keymaps, stdin.into_iter(), stdout);

//     let stdout_str = String::from_utf8(stdout.buffer).unwrap();

//     println!("STDOUT_STR: {:?}", stdout_str);

//     assert!(stdout_str.contains("abc"));
// }
