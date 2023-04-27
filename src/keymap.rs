pub struct Keymap {
    pub key: char,
    pub description: &'static str,
    pub command: &'static str,
    pub prompt: Option<&'static str>,
}

impl Keymap {
    pub fn new(key: char, description: &'static str, command: &'static str) -> Self {
        Keymap {
            key,
            description,
            command,
            prompt: None,
        }
    }
}
