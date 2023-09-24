#[derive(Hash, Eq, Clone)]
pub struct Keybind {
    pub key: String,
    pub modifiers: u32,
    pub command: String
}

impl Keybind {
    pub fn new(key: &str, modifiers: u32, command: &str) -> Self {
        Self {
            key: key.to_string(),
            modifiers,
            command: command.to_string()
        }
    }
}

impl PartialEq for Keybind {
    fn eq(&self, other: &Self) -> bool {
        (self.key.as_str() == other.key.as_str()) && 
        (self.modifiers == other.modifiers)
    }
}