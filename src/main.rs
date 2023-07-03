use keymap::keymap::Keymap;
use keymap::raw_stdout::RawStdout;
use keymap::screen::Screen;
use keymap::step::Step;
use std::io::stdin;
use std::io::Write;
use termion::event::Key;
use termion::input::TermRead;

fn main() {
    let stdout = RawStdout::new().unwrap();
    let screen = Screen::new(stdout);
    let mut step = Step::new(screen);

    step.screen.stdout.flush().unwrap();

    let keymaps = vec![
        Keymap::new(
            't',
            "To-dos",
            "hx ~/Dropbox/DropsyncFiles/markdown/to-dos.md",
            // "hx to-dos.md",
        ),
        Keymap::new('x', "Sleep", "sleep 2 && echo test && sleep 2"),
        Keymap::new('s', "git status", "git status"),
        Keymap::with_prompt(
            'g',
            "Git add and commit",
            "git add . && git commit -m \"{}\"",
            "Enter commit message:",
        ),
        Keymap::new('o', "Open script", "hx script.*"),
        Keymap::new('r', "Run script", "./script.*"),
        Keymap::new('c', "cargo run --release", "cargon run --release"),
        Keymap::new(
            'e',
            "Realesrgan enlarge",
            "/home/Downloads/realesrgan/enlarge.sh",
        ),
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
