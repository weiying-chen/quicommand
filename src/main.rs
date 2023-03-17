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

// TODO: Maybe it should be called KeyHandler
struct InputHandler {
    input: String,
    x: u16,
    y: u16,
}

impl InputHandler {
    fn new(input: String) -> Self {
        Self { input, x: 1, y: 1 }
    }

    fn handle_enter(self) -> Result<Input, InputError> {
        if self.input.trim().is_empty() {
            Err(InputError::EmptyString)
        } else {
            Ok(Input::Text(self.input))
        }
    }

    fn handle_left(&mut self, stdout: &mut impl Write) {
        write!(stdout, "{}", termion::cursor::Left(1)).unwrap();

        let cursor_pos = stdout.cursor_pos().unwrap();

        self.x = cursor_pos.0;
    }

    fn handle_right(&mut self, stdout: &mut impl Write) {
        //TODO: See if can remove these if statements all of the function
        // Or check if if statements in functions are okay
        if self.x <= self.input.len() as u16 {
            write!(stdout, "{}", termion::cursor::Right(1)).unwrap();

            let cursor_pos = stdout.cursor_pos().unwrap();

            self.x = cursor_pos.0;
        }
    }

    fn handle_backspace(&mut self, stdout: &mut impl Write) {
        if self.x > 1 {
            self.x -= 1;
            self.input.remove((self.x - 1).into());

            let cursor_pos = stdout.cursor_pos().unwrap();

            self.y = cursor_pos.1;

            write!(
                stdout,
                "{}{}{}",
                termion::cursor::Goto(1, self.y),
                termion::clear::CurrentLine,
                self.input,
            )
            .unwrap();

            write!(stdout, "{}", termion::cursor::Goto(self.x, self.y)).unwrap();
        }
    }

    fn handle_char(&mut self, stdout: &mut impl Write, c: char) -> Result<(), InputError> {
        let bytes = vec![c as u8];
        std::str::from_utf8(&bytes)
            .map_err(|_| InputError::NotUTF8(bytes.clone()))
            .and_then(|_| {
                self.input.insert((self.x - 1).into(), c);

                let cursor_pos = stdout.cursor_pos().unwrap();

                self.y = cursor_pos.1;

                write!(
                    stdout,
                    "{}{}{}",
                    termion::cursor::Goto(1, self.y),
                    termion::clear::CurrentLine,
                    self.input,
                )
                .unwrap();

                write!(stdout, "{}", termion::cursor::Goto(self.x + 1, self.y)).unwrap();

                let cursor_pos = stdout.cursor_pos().unwrap();

                self.x = cursor_pos.0;

                Ok(())
            })
    }
}

// TODO: Maybe this should be called Input Result or something
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
                write!(stdout, "\r\n").unwrap();
                return;
            }
            Err(e) => {
                write!(stdout, "\r\nInvalid input: {}\r\n", e).unwrap();
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
                        write!(stdout, "{}\r\n", line).unwrap();
                    }
                } else {
                    let stderr_str = String::from_utf8_lossy(&output.stderr);

                    write!(
                        stdout,
                        "Command execution failed: {}\r\n",
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
    let input = String::new();
    // let mut y = 1;
    let mut input_handler = InputHandler::new(input);

    for key in stdin().keys() {
        match key.unwrap() {
            Key::Char('\n') => return input_handler.handle_enter(),
            Key::Esc => return Ok(Input::Exit),
            Key::Char(c) => input_handler.handle_char(stdout, c)?,
            Key::Left => input_handler.handle_left(stdout),
            Key::Right => input_handler.handle_right(stdout),
            Key::Backspace => input_handler.handle_backspace(stdout),
            _ => {}
        }

        stdout.flush().unwrap();
    }

    // This places the output on a new line.
    write!(stdout, "\r\n").unwrap();

    let input = input_handler.input.trim().to_owned();

    Ok(Input::Text(input))
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
