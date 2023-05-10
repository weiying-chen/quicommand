use assert_cmd::Command;
use std::io::{self, Write};

#[test]
fn test_run() {
    let output = Command::new("/home/alex/rust/keymap/target/debug/keymap")
        .output()
        .expect("failed to execute process");

    println!("STATUS: {}", output.status);

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    println!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
    println!("STDERR: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());
}
