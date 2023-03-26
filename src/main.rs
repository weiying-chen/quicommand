use command_launcher::command::Command;
use command_launcher::keymap::Keymap;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}Please select a command:{}\r\n",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide,
    )
    .unwrap();

    let keymaps = vec![
        Keymap {
            key: 'f',
            description: "Feat: adds a new feature to the product",
            command: "git add . && git commit -m 'Feat: {}'",
            input_placeholder: "{}",
        },
        Keymap {
            key: 'x',
            description: "Fix: fixes a defect in a feature",
            command: "git add . && git commit -m 'Fix: {}'",
            input_placeholder: "{}",
        },
        Keymap {
            key: 'r',
            description: "Refac: changes a feature's code but not its behavior",
            command: "git add . && git commit -m 'Refac: {}'",
            input_placeholder: "{}",
        },
        Keymap {
            key: 'd',
            description: "Docs: changes related to documentation",
            command: "git add . && git commit -m 'Docs: {}'",
            input_placeholder: "{}",
        },
        Keymap {
            key: 's',
            description: "Run git status",
            command: "git status",
            input_placeholder: "{}",
        },
    ];

    for keymap in &keymaps {
        write!(stdout, "{}  {}\r\n", keymap.key, keymap.description).unwrap();
    }

    stdout.flush().unwrap();

    for key in stdin().keys() {
        match key.unwrap() {
            // TODO: Esc should perform the same action.
            Key::Char('q') => {
                write!(stdout, "{}", termion::cursor::Show).unwrap();
                break;
            }
            Key::Char(c) if keymaps.iter().any(|k| k.key == c) => {
                let keymap = keymaps.iter().find(|k| k.key == c).unwrap();

                write!(stdout, "{}Enter commit message: ", termion::cursor::Show).unwrap();
                stdout.flush().unwrap();

                let input_text = command_launcher::input::get_input(stdin().keys(), &mut stdout);
                let handled_input = command_launcher::input::handle_input(input_text, &mut stdout);

                // TODO: the command should return a result
                let mut command = Command::new(keymap.command, &handled_input);
                command.execute(&mut stdout);
                break;
            }
            _ => {}
        }
    }
}
