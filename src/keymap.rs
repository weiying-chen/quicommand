pub struct Keymap {
    pub key: char,
    pub description: String,
    pub command: String,
    pub prompt: Option<String>,
}

impl Keymap {
    pub fn new(key: char, description: String, command: String) -> Self {
        Self {
            key,
            description,
            command,
            prompt: None,
        }
    }

    pub fn with_prompt(key: char, description: String, command: String, prompt: String) -> Self {
        Self {
            key,
            description,
            command,
            prompt: Some(prompt),
        }
    }
}
