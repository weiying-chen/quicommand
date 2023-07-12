use keymap::{
    keymap::Keymap,
    mock_stdout::MockStdout,
    screen::Screen,
    step::{Process, Step},
};

use termion::event::Key;

fn get_keymaps<'a>() -> Vec<Keymap> {
    vec![Keymap::new('t', "echo 'test'")]
}

fn get_keymaps_with_description<'a>() -> Vec<Keymap> {
    vec![Keymap::new('t', "echo {}").with_description("Test description")]
}

fn get_keymaps_with_prompt<'a>() -> Vec<Keymap> {
    vec![Keymap::new('t', "echo {}").with_prompt("Test prompt")]
}

fn setup_step() -> Step<MockStdout> {
    let stdout = MockStdout::new();
    let screen = Screen::new(stdout);
    let step = Step::new(screen);
    step
}

#[test]
fn keymap() {
    let keymaps = get_keymaps();
    let mut step = setup_step();

    step.show_select_command(&keymaps);

    let stdout_str = String::from_utf8(step.screen.stdout.buffer).unwrap();
    let has_prompt = stdout_str.contains("Please select a command:");
    let has_menu = stdout_str.contains("t  echo 'test'");

    assert!(has_prompt && has_menu);
}

#[test]
fn keymap_with_description() {
    let keymaps = get_keymaps_with_description();
    let mut step = setup_step();

    step.show_select_command(&keymaps);

    let stdout_str = String::from_utf8(step.screen.stdout.buffer).unwrap();
    let has_prompt = stdout_str.contains("Please select a command:");
    let has_menu = stdout_str.contains("Test description");

    assert!(has_prompt && has_menu);
}

#[test]
fn command() {
    let mut step = setup_step();
    let keys = Vec::new();
    let input = step.input_from_prompt(None, keys.into_iter());
    let keymap = get_keymaps().into_iter().next().unwrap();
    let output = step.process_input(input, &keymap);
    let result = output.unwrap();

    let Process::Output(output) = result else {
      panic!();
    };

    let stdout_str = String::from_utf8_lossy(&output.stdout);

    assert_eq!(stdout_str, "test");
}

#[test]
fn command_with_input() {
    let mut step = setup_step();
    let keymap = get_keymaps_with_prompt().into_iter().next().unwrap();

    let keys = vec![
        Ok(Key::Char('t')),
        Ok(Key::Char('e')),
        Ok(Key::Char('s')),
        Ok(Key::Char('t')),
    ];

    let input = step.input_from_prompt(keymap.prompt.as_deref(), keys.into_iter());
    let output = step.process_input(input, &keymap);
    let result = output.unwrap();

    let Process::Output(output) = result else {
      panic!();
    };

    let stdout_str = String::from_utf8_lossy(&output.stdout);

    assert_eq!(stdout_str, "test");
}

#[test]
fn command_with_prompt() {
    let mut step = setup_step();
    let keymap = get_keymaps_with_prompt().into_iter().next().unwrap();
    let keys = Vec::new();

    step.input_from_prompt(keymap.prompt.as_deref(), keys.into_iter())
        .unwrap();

    let stdout_str = String::from_utf8(step.screen.stdout.buffer).unwrap();

    assert!(stdout_str.contains("Test prompt"));
}

#[test]
fn command_with_empty_input() {
    let mut step = setup_step();
    let keymap = get_keymaps_with_prompt().into_iter().next().unwrap();
    let keys = vec![Ok(Key::Char('\n'))];
    let input = step.input_from_prompt(keymap.prompt.as_deref(), keys.into_iter());
    let output = step.process_input(input, &keymap);

    let Err(input_error) = output else {
        panic!();
    };

    assert_eq!(input_error.to_string(), "Input was empty");
}

#[test]
fn command_with_cancelled_input() {
    let mut step = setup_step();
    let keymap = get_keymaps_with_prompt().into_iter().next().unwrap();
    let keys = vec![Ok(Key::Esc)];
    let input = step.input_from_prompt(keymap.prompt.as_deref(), keys.into_iter());
    let output = step.process_input(input, &keymap);

    assert!(matches!(output.unwrap(), Process::Exit));
}
