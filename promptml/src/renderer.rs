use std::collections::HashMap;

use crate::error::PromptError;
use crate::message::{Message, Role};
use crate::parser::{parse, Token};
use crate::template::PromptTemplate;

/// A set of key-value variables for one row of a few-shot examples block.
pub struct Example {
    pub vars: HashMap<String, String>,
}

/// A builder that accumulates variable bindings before rendering a [`PromptTemplate`].
pub struct RenderBuilder<'a> {
    template: &'a PromptTemplate,
    vars: HashMap<String, String>,
    examples: Vec<Example>,
}

impl<'a> RenderBuilder<'a> {
    pub(crate) fn new(template: &'a PromptTemplate) -> Self {
        Self {
            template,
            vars: HashMap::new(),
            examples: Vec::new(),
        }
    }

    pub fn set(mut self, key: &str, val: &str) -> Self {
        self.vars.insert(key.to_string(), val.to_string());
        self
    }

    /// Supply the examples rendered inside `{{#examples}}` blocks.
    pub fn examples(mut self, ex: Vec<Example>) -> Self {
        self.examples = ex;
        self
    }

    /// Validate all required variables and render to a string.
    pub fn build(self) -> Result<String, PromptError> {
        self.validate_required()?;
        render_tokens(&self.template.tokens, &self.vars, &self.examples)
    }

    /// Render into a `Vec<Message>` suitable for chat APIs.
    ///
    /// Produces a `System` message when a system template is present, followed
    /// by a `User` message for the main template body.
    pub fn to_messages(self) -> Result<Vec<Message>, PromptError> {
        self.validate_required()?;
        let mut messages = Vec::new();

        if let Some(sys_tokens) = &self.template.system_tokens {
            let content = render_tokens(sys_tokens, &self.vars, &self.examples)?;
            messages.push(Message { role: Role::System, content });
        }

        let content = render_tokens(&self.template.tokens, &self.vars, &self.examples)?;
        messages.push(Message { role: Role::User, content });

        Ok(messages)
    }

    fn validate_required(&self) -> Result<(), PromptError> {
        for var in &self.template.required_vars {
            if !self.vars.contains_key(var) {
                return Err(PromptError::MissingVariable(var.clone()));
            }
        }
        Ok(())
    }
}

// Walks `tokens` and writes rendered output into a String.
// skip_depth handles nested {{#if}} blocks: positive depth means we're inside
// a block whose condition was false.
pub(crate) fn render_tokens(
    tokens: &[Token],
    vars: &HashMap<String, String>,
    examples: &[Example],
) -> Result<String, PromptError> {
    let mut out = String::new();
    let mut skip_depth: u32 = 0;

    for token in tokens {
        match token {
            Token::IfStart(var) => {
                if skip_depth > 0 || !vars.contains_key(var.as_str()) {
                    skip_depth += 1;
                }
            }
            Token::IfEnd => {
                skip_depth = skip_depth.saturating_sub(1);
            }
            _ if skip_depth > 0 => {}
            Token::Text(t) => out.push_str(t),
            Token::Variable(name) => {
                let val = vars
                    .get(name)
                    .ok_or_else(|| PromptError::MissingVariable(name.clone()))?;
                out.push_str(val);
            }
            Token::ExamplesBlock(inner) => {
                let inner_tokens = parse(inner)?;
                for example in examples {
                    let rendered = render_tokens(&inner_tokens, &example.vars, &[])?;
                    out.push_str(&rendered);
                    out.push('\n');
                }
            }
        }
    }

    Ok(out)
}
