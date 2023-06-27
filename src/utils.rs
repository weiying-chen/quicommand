pub fn escape_backticks(input: &str) -> String {
    let mut result = String::new();

    for c in input.chars() {
        if c == '`' {
            result.push('\\');
        }
        result.push(c);
    }
    result
}
