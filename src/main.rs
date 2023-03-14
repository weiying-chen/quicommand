use std::fmt;
use std::io::{stdin, stdout, Write};
use std::process::Command;
use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

struct KeyboardShortcut {
    key: char,
    description: &'static str,
    command: &'static str,
    input_placeholder: &'static str,
}

enum Input {
    Text(String),
    Exit,
}

enum InputError {
    NotUTF8(Vec<u8>),
    EmptyString,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputError::NotUTF8(bytes) => write!(
                f,
                "Input contained non-UTF8 bytes: {:?}",
                bytes
                    .iter()
                    .map(|b| format!("0x{:X}", b))
                    .collect::<Vec<_>>()
            ),
            InputError::EmptyString => write!(f, "Input was empty."),
        }
    }
}

impl KeyboardShortcut {
    fn execute_command(&self, stdout: &mut impl Write) {
        write!(stdout, "Enter commit message: ").unwrap();
        stdout.flush().unwrap();

        let input = match get_input(stdout) {
            Ok(Input::Text(i)) => i,
            Ok(Input::Exit) => {
                write!(stdout, "\n\r").unwrap();
                return;
            }
            Err(e) => {
                write!(stdout, "\n\rInvalid input: {}\n\r", e).unwrap();
                return;
            }
        };

        let command = self.command.replace(self.input_placeholder, &input);

        // This combination makes commands print colors.
        let output = Command::new("script")
            .arg("-qec")
            .arg(command)
            .arg("/dev/null")
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    let output_str = String::from_utf8_lossy(&output.stdout);

                    for line in output_str.lines() {
                        write!(stdout, "{}\n\r", line).unwrap();
                    }
                } else {
                    let stderr_str = String::from_utf8_lossy(&output.stderr);
                    write!(
                        stdout,
                        "Command execution failed: {}\n\r",
                        stderr_str.trim()
                    )
                    .unwrap();
                }
            }
            Err(e) => {
                write!(stdout, "Error executing command: {:?}\r\n", e).unwrap();
            }
        }
    }
}

fn get_input(stdout: &mut impl Write) -> Result<Input, InputError> {
    let mut input = String::new();

    for key in stdin().keys() {
        match key.unwrap() {
            // This is Enter.
            Key::Char('\n') => {
                if input.trim().is_empty() {
                    return Err(InputError::EmptyString);
                } else {
                    break;
                }
            }
            Key::Char(c) => {
                let bytes = vec![c as u8];
                std::str::from_utf8(&bytes)
                    // `bytes` has to be cloned to prevent a move error
                    .map_err(|_| InputError::NotUTF8(bytes.clone()))
                    .and_then(|_| {
                        input.push(c);
                        write!(stdout, "{}", c).unwrap();
                        Ok(())
                    })?;
            }
            // Key::Char(c) => {
            //     let bytes = vec![c as u8];
            //     match std::str::from_utf8(&bytes) {
            //         Ok(_) => {
            //             input.push(c);
            //             write!(stdout, "{}", c).unwrap();
            //             Ok(())
            //         }
            //         Err(_) => Err(InputError::NotUTF8(bytes)),
            //     }?;
            // }
            Key::Esc => {
                // return Err(InputError::NotUTF8(vec![0x1b]));
                return Ok(Input::Exit);
            }
            Key::Left => {
                write!(stdout, "{}", termion::cursor::Left(1)).unwrap();
            }
            Key::Right => {
                write!(stdout, "{}", termion::cursor::Right(1)).unwrap();
            }
            Key::Backspace => {
                if input.is_empty() {
                    continue;
                }

                let cursor_pos = stdout.cursor_pos().unwrap();
                let x = cursor_pos.0;
                let y = cursor_pos.1;

                // let input_chars: Vec<char> = input.chars().collect();
                // let input_len = input_chars.len();

                // let (width, _) = termion::terminal_size().unwrap();

                // let offset = y * width + x; // calculate the offset of the cursor position

                // let index = usize::from(offset).saturating_sub(input_len);

                // let corresponding_char = input_chars.get(index).unwrap_or(&'\0');

                // println!("{:?}", offset);

                // let char_idx = (y - 1) * (width as u16) + x - 2;
                // println!("\r\nchar_idx: {:?}", char_idx);
                // if usize::from(char_idx) < input_len {
                // input_chars.remove(char_idx.into());
                // }

                // let new_input = input_chars.into_iter().collect::<String>();

                // println!("\r\nnew_input: {:?}", new_input);

                // let cursor_pos = stdout.cursor_pos().unwrap();

                // println!("Cursor pos: {:?}", cursor_pos);

                // let x = cursor_pos.0;
                // let y = cursor_pos.1;

                // println!("Input length: {}", input.len());
                // input.remove(x as usize - 1);
                // write!(stdout, "{}{}", termion::cursor::Goto(1, y), input).unwrap();

                // remove character on the left side of the cursor

                // let mut chars = input.chars();
                // chars.next_back();
                // let audited_line = chars.as_str();
                // let new_input = audited_line.to_string();

                // write!(
                //     stdout,
                //     "{}{}{}",
                //     termion::cursor::Left(1),
                //     termion::clear::CurrentLine,
                //     audited_line,
                // )
                // .unwrap();

                // write!(stdout, "{}\r", termion::clear::CurrentLine).unwrap();
                // write!(stdout, "{}", audited_line).unwrap();

                // input = new_input;
            }
            _ => {}
        }

        stdout.flush().unwrap();
    }

    let input = input.trim().to_owned();

    Ok(Input::Text(input))
}

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}Please select a command:\r\n{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide,
    )
    .unwrap();

    let keyboard_shortcuts = vec![
        KeyboardShortcut {
            key: 'f',
            description: "Feat: adds a new feature to the product",
            command: "git add . && git commit -m 'Feat: {}'",
            input_placeholder: "{}",
        },
        KeyboardShortcut {
            key: 'x',
            description: "Fix: fixes a defect in a feature",
            command: "git add . && git commit -m 'Fix: {}'",
            input_placeholder: "{}",
        },
        KeyboardShortcut {
            key: 'r',
            description: "Refac: changes a feature's code but not its behavior",
            command: "git add . && git commit -m 'Refac: {}'",
            input_placeholder: "{}",
        },
        KeyboardShortcut {
            key: 'c',
            description: "Chore: changes that are not related any feature",
            // command: "git add . && git commit -m 'Chore: {}'",
            command: "git status",
            input_placeholder: "{}",
        },
    ];

    for keyboard_shortcut in &keyboard_shortcuts {
        write!(
            stdout,
            "{}  {}\r\n",
            keyboard_shortcut.key, keyboard_shortcut.description
        )
        .unwrap();
    }

    stdout.flush().unwrap();

    for key in stdin().keys() {
        match key.unwrap() {
            // TODO: Esc should perform the same action.
            Key::Char('q') => {
                write!(stdout, "{}", termion::cursor::Show).unwrap();
                break;
            }
            Key::Char(c) if keyboard_shortcuts.iter().any(|k| k.key == c) => {
                let keyboard_shortcut = keyboard_shortcuts.iter().find(|k| k.key == c).unwrap();

                // Raw mode has to be suspented to collect input.
                // stdout.suspend_raw_mode().unwrap();
                write!(stdout, "{}", termion::cursor::Show).unwrap();
                keyboard_shortcut.execute_command(&mut stdout);
                // stdout.activate_raw_mode().unwrap();
                break;
            }
            _ => {}
        }
    }
}
