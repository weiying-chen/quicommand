use quicommand::keymap::Keymap;
use quicommand::raw_stdout::RawStdout;
use quicommand::screen::Screen;
use quicommand::step::Step;
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
        Keymap::new('c', "git add . && git commit -m \"{}\"")
            .with_description("Git commit")
            .with_prompt("Enter commit message:"),
        Keymap::new('m', "hx src/main.*"),
        Keymap::new('n', "node script.*"),
        Keymap::new('b', "cargo build --release"),
        Keymap::new(
            't',
            "git log --author=\"Alex\" --since=\"midnight\" --no-merges --oneline | wc -l",
        ),
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
