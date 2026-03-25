/// Errors that can occur when working with prompt templates.
#[derive(Debug, thiserror::Error)]
pub enum PromptError {
    /// A variable name referenced in the template is not recognised.
    #[error("unknown variable: {0}")]
    UnknownVariable(String),

    /// A required variable was not supplied before calling `build`.
    #[error("missing variable: {0}")]
    MissingVariable(String),

    #[error("parse error: {0}")]
    ParseError(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    TomlError(#[from] toml::de::Error),
}
