use std::io::Write;

use keymap::{
    keymap::Keymap,
    screen::Screen,
    step::{Process, Step},
    term_writer::TermCursor,
};
use termion::event::Key;

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
    let screen = Screen::new(stdout);
    let mut step = Step::new(screen);

    step.show_select_command(&menu_items);

    let stdout_str = String::from_utf8(step.screen.stdout.buffer).unwrap();
    let has_prompt = stdout_str.contains("Please select a command:");
    let has_menu = stdout_str.contains("t  Test keymap");

    assert!(has_prompt && has_menu);
}

#[test]
fn command() {
    let keymaps = get_keymaps();
    let stdout = MockStdout::new();
    let screen = Screen::new(stdout);
    let mut step = Step::new(screen);
    let keys = Vec::new();
    let input = step.input_from_prompt(None, keys.into_iter());
    let output = step.process_input(input, &keymaps[0]);
    let result = output.unwrap();

    println!("result: {:?}", result);

    let Process::Output(output) = result else {
      panic!();
    };

    let stdout_str = String::from_utf8_lossy(&output.stdout);

    assert_eq!(stdout_str, "test");
}

#[test]
fn command_with_input() {
    let keymaps = get_keymaps_with_prompt();
    let stdout = MockStdout::new();
    let screen = Screen::new(stdout);
    let mut step = Step::new(screen);

    let keys = vec![
        Ok(Key::Char('t')),
        Ok(Key::Char('e')),
        Ok(Key::Char('s')),
        Ok(Key::Char('t')),
    ];

    let input = step.input_from_prompt(keymaps[0].prompt.clone(), keys.into_iter());
    let output = step.process_input(input, &keymaps[0]);
    let result = output.unwrap();

    let Process::Output(output) = result else {
      panic!();
    };

    let stdout_str = String::from_utf8_lossy(&output.stdout);

    assert_eq!(stdout_str, "test");
}

#[test]
fn command_with_input_prompt() {
    let keymaps = get_keymaps_with_prompt();
    let stdout = MockStdout::new();
    let screen = Screen::new(stdout);
    let mut step = Step::new(screen);

    let keys = Vec::new();

    step.input_from_prompt(keymaps[0].prompt.clone(), keys.into_iter())
        .unwrap();

    let stdout_str = String::from_utf8(step.screen.stdout.buffer).unwrap();

    assert!(stdout_str.contains("Test prompt"));
}

#[test]
fn command_with_empty_input() {
    let keymaps = get_keymaps_with_prompt();
    let stdout = MockStdout::new();
    let screen = Screen::new(stdout);
    let mut step = Step::new(screen);
    let keys = vec![Ok(Key::Char('\n'))];
    let input = step.input_from_prompt(keymaps[0].prompt.clone(), keys.into_iter());
    let output = step.process_input(input, &keymaps[0]);

    let Err(input_error) = output else {
        panic!();
    };

    assert_eq!(input_error.to_string(), "Input was empty");
}

#[test]
fn command_with_cancelled_input() {
    let keymaps = get_keymaps_with_prompt();
    let stdout = MockStdout::new();
    let screen = Screen::new(stdout);
    let mut step = Step::new(screen);
    let keys = vec![Ok(Key::Esc)];
    let input = step.input_from_prompt(keymaps[0].prompt.clone(), keys.into_iter());
    let output = step.process_input(input, &keymaps[0]);

    assert!(matches!(output.unwrap(), Process::Exit));
}
