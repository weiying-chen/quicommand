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
        Keymap::new('a', "sleep 2 && echo test && sleep 2").with_description("Just a test"),
        Keymap::new('b', "sleep 2 && echo test && sleep 2"),
        Keymap::new('t', "hx ~/Dropbox/DropsyncFiles/markdown/to-dos.md")
            .with_description("To-dos"),
        Keymap::new('x', "sleep 2 && echo test && sleep 2"),
        Keymap::new('s', "git status"),
        Keymap::new('g', "git add . && git commit -m \"{}\"")
            .with_description("Git commit")
            .with_prompt("Enter commit message:"),
        Keymap::new('o', "hx script.*"),
        Keymap::new('r', "./script.*"),
        Keymap::new('c', "cargon run --release"),
    ];

    step.show_select_cmd(&keymaps);
    // screen.stdout.flush().unwrap();

    for key in stdin().keys() {
        match key.unwrap() {
            Key::Char('q') => {
                step.screen.show_cursor();
                break;
            }
            Key::Char(key) => {
                let Some(keymap) = keymaps.iter().find(|k| k.key == key) else {
                   continue;
                };

                let input = step.input_from_prompt(keymap.prompt.as_deref(), stdin().keys());

                step.process_input(input, keymap).unwrap();
                break;
            }
            _ => {}
        }
    }
}
