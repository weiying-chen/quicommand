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

pub fn starts_with_any(s: &str, pats: &[&str]) -> bool {
    pats.iter().any(|pat| s.starts_with(pat))
}
