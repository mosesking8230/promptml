use serde::Deserialize;

use crate::error::PromptError;
use crate::template::PromptTemplate;

#[derive(Deserialize)]
struct TomlFile {
    messages: MessagesSection,
}

#[derive(Deserialize)]
struct MessagesSection {
    system: Option<String>,
    user: String,
}

impl PromptTemplate {
    /// Load a template from a TOML file on disk.
    ///
    /// The file must contain a `[messages]` section with a required `user`
    /// string and an optional `system` string.
    pub fn from_file(path: &str) -> Result<Self, PromptError> {
        let content = std::fs::read_to_string(path)?;
        let file: TomlFile = toml::from_str(&content)?;
        Self::new_with_system(&file.messages.user, file.messages.system.as_deref())
    }
}
