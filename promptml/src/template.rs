use crate::error::PromptError;
use crate::parser::{parse, Token};
use crate::renderer::RenderBuilder;

/// A parsed prompt template that can be rendered with variable substitution.
pub struct PromptTemplate {
    pub(crate) tokens: Vec<Token>,
    pub(crate) system_tokens: Option<Vec<Token>>,
    /// Variables that must be supplied before calling `build`.
    ///
    /// Only contains variables found *outside* conditional `{{#if}}` blocks.
    pub(crate) required_vars: Vec<String>,
}

impl PromptTemplate {
    pub fn new(template: &str) -> Result<Self, PromptError> {
        let tokens = parse(template)?;
        let required_vars = extract_required_vars(&tokens);
        Ok(Self {
            tokens,
            system_tokens: None,
            required_vars,
        })
    }

    /// Parse a user template and an optional system template.
    pub fn new_with_system(user: &str, system: Option<&str>) -> Result<Self, PromptError> {
        let tokens = parse(user)?;
        let mut required_vars = extract_required_vars(&tokens);

        let system_tokens = match system {
            Some(sys) => {
                let sys_tokens = parse(sys)?;
                for v in extract_required_vars(&sys_tokens) {
                    if !required_vars.contains(&v) {
                        required_vars.push(v);
                    }
                }
                Some(sys_tokens)
            }
            None => None,
        };

        Ok(Self {
            tokens,
            system_tokens,
            required_vars,
        })
    }

    /// Called by the `prompt!` macro with a compile-time-validated template.
    ///
    /// Panics if `template` cannot be parsed, which cannot happen for a string
    /// that was already checked at compile time.
    #[doc(hidden)]
    pub fn new_validated(template: &str, vars: &[&str]) -> Self {
        let tokens = parse(template).expect("proc-macro validated template");
        Self {
            tokens,
            system_tokens: None,
            required_vars: vars.iter().map(|v| (*v).to_string()).collect(),
        }
    }

    pub fn render(&self) -> RenderBuilder<'_> {
        RenderBuilder::new(self)
    }
}

fn extract_required_vars(tokens: &[Token]) -> Vec<String> {
    let mut vars: Vec<String> = Vec::new();
    let mut depth: u32 = 0;

    for token in tokens {
        match token {
            Token::IfStart(_) => depth += 1,
            Token::IfEnd => depth = depth.saturating_sub(1),
            Token::Variable(name) if depth == 0 => {
                if !vars.contains(name) {
                    vars.push(name.clone());
                }
            }
            _ => {}
        }
    }

    vars
}
