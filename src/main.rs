use keymap::keymap::Keymap;
use keymap::screen::Screen;
use keymap::step::Step;
use keymap::term_writer::TermCursor;
use std::io::{stdin, Write};
// use std::sync::{Arc, Mutex};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

//To-do: move this into its own file.

struct RawStdout {
    buffer: RawTerminal<std::io::Stdout>,
}

impl RawStdout {
    pub fn new() -> std::io::Result<Self> {
        let buffer = std::io::stdout().into_raw_mode()?;
        Ok(Self { buffer })
    }
}

impl Write for RawStdout {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buffer.flush()
    }
}

impl TermCursor for RawStdout {
    fn write_term(&mut self, fmt: std::fmt::Arguments) -> std::io::Result<()> {
        std::io::Write::write_fmt(self, fmt)
    }

    fn get_cursor_pos(&mut self) -> Result<(u16, u16), std::io::Error> {
        termion::cursor::DetectCursorPos::cursor_pos(self)
    }
}

fn main() {
    let stdout = RawStdout::new().unwrap();
    let screen = Screen::new(stdout);
    let mut step = Step::new(screen);

    step.screen.stdout.flush().unwrap();

    let keymaps = vec![
        Keymap::new('t', "Sleep", "sleep 2 && echo test && sleep 2"),
        Keymap::with_prompt(
            'c',
            "Git add and commit",
            "git add . && git commit -m \"{}\"",
            "Enter commit message:",
        ),
        Keymap::new('o', "Open script", "vi script.txt"),
        Keymap::new('s', "Run script.sh", "./script.sh"),
        Keymap::new('r', "cargo run --release", "cargon run --release"),
        Keymap::new(
            'a',
            "Run all",
            "/home/alex/bash/crop/script.sh &&
            /home/alex/rust/visual-center/target/release/visual_center &&
            /home/alex/bash/delete/script.sh",
        ),
    ];

    let menu_items: Vec<String> = keymaps
        .iter()
        .map(|keymap| format!("{}  {}", keymap.key, keymap.description))
        .collect();

    step.show_select_command(&menu_items);
    // screen.stdout.flush().unwrap();

    for key in stdin().keys() {
        match key.unwrap() {
            Key::Char('q') => {
                step.screen.show_cursor();
                break;
            }
            Key::Char(key) => {
                if let Some(keymap) = keymaps.iter().find(|k| k.key == key) {
                    let input = step.input_from_prompt(keymap.prompt.clone(), stdin().keys());

                    step.process_input(input, keymap).unwrap();
                    break;
                }
            }
            _ => {}
        }
    }
}
