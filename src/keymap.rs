#[derive(Debug, Default, Clone)]
pub struct Keymap {
    pub key: char,
    pub command: String,
    pub description: String,
    pub prompt: Option<String>,
}

impl Keymap {
    pub fn new<S: AsRef<str>>(key: char, command: S) -> Self {
        let command = command.as_ref().to_owned();
        let description = command.clone();

        Self {
            key,
            command,
            description,
            ..Default::default()
        }
    }

    pub fn with_prompt<S: AsRef<str>>(mut self, prompt: S) -> Self {
        self.prompt = Some(prompt.as_ref().to_owned());
        self
    }

    pub fn with_description<S: AsRef<str>>(mut self, description: S) -> Self {
        self.description = description.as_ref().to_owned();
        self
    }
}
