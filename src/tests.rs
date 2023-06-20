use super::*;

// Stdout

#[derive(Debug)]
struct MockStdout {
    buffer: Vec<u8>,
    cursor_pos: (u16, u16),
}

impl MockStdout {
    pub fn new() -> Self {
        let buffer = Vec::new();

        Self {
            buffer,
            cursor_pos: (1, 1),
        }
    }
}

impl Write for MockStdout {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buffer.flush()
    }
}

impl TermCursor for MockStdout {
    fn write_term(&mut self, fmt: std::fmt::Arguments) -> std::io::Result<()> {
        const INPUT_START: &str = "\u{1b}[2K";

        if fmt.to_string().contains(INPUT_START) {
            self.cursor_pos.0 += 1;
        }

        self.write_fmt(fmt).unwrap();

        Ok(())
    }

    fn get_cursor_pos(&mut self) -> Result<(u16, u16), std::io::Error> {
        Ok(self.cursor_pos)
    }
}

fn get_keymaps<'a>() -> Vec<Keymap> {
    vec![Keymap::new('t', "Test keymap", "echo 'test'")]
}

fn get_keymaps_with_prompt<'a>() -> Vec<Keymap> {
    vec![Keymap::with_prompt(
        't',
        "Test keymap",
        "echo {}",
        "Test prompt",
    )]
}

#[test]
fn test_show_keymap_menu() {
    let keymaps = get_keymaps();

    let menu_items: Vec<String> = keymaps
        .iter()
        .map(|keymap| format!("{}  {}", keymap.key, keymap.description))
        .collect();

    let stdout = MockStdout::new();
    let mut screen = Screen::new(stdout);

    screen.show_menu(&menu_items);

    let stdout_str = String::from_utf8(screen.stdout.buffer).unwrap();

    assert!(stdout_str.contains("t  Test keymap"));
}

// #[test]
// fn test_show_input_instruction() {
//     const PROMPT_MESSAGE: &str = "Enter commit message:";
//     let mut stdout = MockStdout::new();

//     prompt_input(PROMPT_MESSAGE, &mut stdout);

//     let stdout_str = String::from_utf8(stdout.buffer).unwrap();

//     assert!(stdout_str.contains(PROMPT_MESSAGE));
// }

// Note: This is already testing run command

#[test]
fn command_without_input() {
    let keymaps = get_keymaps();
    let stdout = MockStdout::new();
    let screen = Screen::new(stdout);
    let mut input_handler = InputHandler::new(screen);
    let keys = Vec::new();
    let input = input_handler.input_from_prompt(None, keys.into_iter());
    let output = input_handler.process_input(input, &keymaps[0]);
    let result = output.unwrap();

    let Process::Output(stdout, _) = result else {
      panic!();
    };

    assert_eq!(stdout, "test")
}

// #[test]
// fn command_with_empty_input() {
//     let keymaps = get_keymaps_with_prompt();
//     let stdout = MockStdout::new();
//     let screen = Screen::new(stdout);
//     let mut input_handler = InputHandler::new(screen);
//     let keys = vec![Ok(Key::Char('\n'))];
//     let input = input_handler.input_from_prompt(keymaps[0].prompt.clone(), keys.into_iter());
//     let output = input_handler.process_input(input, &keymaps[0]);

//     assert!(matches!(output, Err(InputError::EmptyString)));
// }

// To-do: this doesn't have an assertion
// #[test]
// fn test_exit_proccess() {
//     let keymaps = get_keymaps_with_prompt();
//     let stdout = MockStdout::new();
//     let screen = Screen::new(stdout);
//     let mut input_handler = InputHandler::new(screen);
//     let keys = vec![Ok(Key::Esc)];
//     let input = input_handler.input_from_prompt(keymaps[0].prompt.clone(), keys.into_iter());
//     let output = input_handler.process_input(input, &keymaps[0]);

//     assert_eq!(output.unwrap(), Process::Exit);
// }

// #[test]
// fn test_run_commmand_with_prompt() {
//     let keymaps = get_keymaps_with_prompt();
//     let mut command = CmdRunner::new(keymaps[0].command.clone(), Some("test".to_string()));
//     let output = command.run_with_output().unwrap();
//     let stdout_str = String::from_utf8(output.stdout).unwrap();

//     assert!(stdout_str.contains("test"));
// }
