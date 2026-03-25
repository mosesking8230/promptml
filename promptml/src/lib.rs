#![doc = include_str!("../README.md")]

pub use error::PromptError;
pub use message::{Message, Role};
pub use promptml_macros::{chat_prompt, prompt};
pub use renderer::{Example, RenderBuilder};
pub use template::PromptTemplate;

mod error;
mod loader;
mod message;
mod parser;
mod renderer;
mod template;
