pub struct Keymap {
    pub key: char,
    pub description: String,
    pub command: String,
    pub prompt: Option<String>,
}

impl Keymap {
    pub fn new(key: char, description: &str, command: &str) -> Self {
        Self {
            key,
            description: description.to_string(),
            command: command.to_string(),
            prompt: None,
        }
    }

    pub fn with_prompt(key: char, description: &str, command: &str, prompt: &str) -> Self {
        Self {
            key,
            description: description.to_string(),
            command: command.to_string(),
            prompt: Some(prompt.to_string()),
        }
    }
}
