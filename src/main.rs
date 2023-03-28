use command_launcher::cmd_runner::CmdRunner;
use command_launcher::input::Input;
use command_launcher::keymap::Keymap;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn handle_quit(mut stdout: impl Write) {
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

// To-do: make this function more reusable.
fn handle_command(key: char, keymaps: &[Keymap], mut stdout: impl Write) {
    if let Some(keymap) = keymaps.iter().find(|k| k.key == key) {
        write!(stdout, "{}Enter commit message: ", termion::cursor::Show).unwrap();
        stdout.flush().unwrap();

        let input = command_launcher::input::get_input(stdin().keys(), &mut stdout);

        match input {
            Ok(Input::Text(i)) => {
                let mut command = CmdRunner::new(keymap.command, &i);
                // To-do: the command should return a result?
                command.execute(&mut stdout);
            }
            Ok(Input::Exit) => {
                write!(stdout, "\r\n").unwrap();
            }
            Err(e) => {
                write!(stdout, "\r\nInvalid input: {}\r\n", e).unwrap();
            }
        };
    }
}

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
            Key::Char('q') => {
                handle_quit(&mut stdout);
                break;
            }
            Key::Char(c) => {
                handle_command(c, &keymaps, &mut stdout);
                break;
            }
            _ => {}
        }
    }
}
