pub struct Keymap<'a> {
    pub key: char,
    pub description: &'a str,
    pub command: &'a str,
    pub prompt: Option<&'a str>,
}

impl<'a> Keymap<'a> {
    pub fn new(key: char, description: &'a str, command: &'a str) -> Self {
        Self {
            key,
            description,
            command,
            prompt: None,
        }
    }

    pub fn with_prompt(key: char, description: &'a str, command: &'a str, prompt: &'a str) -> Self {
        Self {
            key,
            description,
            command,
            prompt: Some(prompt),
        }
    }
}
