use crate::error::PromptError;

// TODO: support `\{` escape sequences for literal braces

/// A single unit produced by the template tokenizer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// A literal text segment.
    Text(String),
    /// A `{name}` variable substitution.
    Variable(String),
    /// The opening of an `{{#if name}}` conditional block.
    IfStart(String),
    /// The closing `{{/if}}` of a conditional block.
    IfEnd,
    /// An `{{#examples}}…{{/examples}}` block; stores the inner template string.
    ExamplesBlock(String),
}

/// Tokenize a template string into a sequence of [`Token`]s.
///
/// Returns an error if the template contains a syntax problem such as an
/// unclosed brace or an empty variable name.
pub fn parse(input: &str) -> Result<Vec<Token>, PromptError> {
    let mut tokens = Vec::new();
    let mut rest = input;

    while !rest.is_empty() {
        if rest.starts_with("{{") {
            let close = rest[2..]
                .find("}}")
                .ok_or_else(|| PromptError::ParseError("unclosed block directive `{{`".into()))?;
            let directive = rest[2..2 + close].trim();
            let after = &rest[2 + close + 2..];

            if let Some(var) = directive.strip_prefix("#if ") {
                let var = var.trim();
                if var.is_empty() {
                    return Err(PromptError::ParseError(
                        "`{{#if}}` requires a variable name".into(),
                    ));
                }
                tokens.push(Token::IfStart(var.to_string()));
                rest = after;
            } else if directive == "/if" {
                tokens.push(Token::IfEnd);
                rest = after;
            } else if directive == "#examples" {
                const END: &str = "{{/examples}}";
                let end_pos = after.find(END).ok_or_else(|| {
                    PromptError::ParseError("unclosed `{{#examples}}` block".into())
                })?;
                let inner = after[..end_pos].to_string();
                tokens.push(Token::ExamplesBlock(inner));
                rest = &after[end_pos + END.len()..];
            } else if directive == "/examples" {
                return Err(PromptError::ParseError(
                    "unexpected `{{/examples}}` without opening `{{#examples}}`".into(),
                ));
            } else {
                return Err(PromptError::ParseError(format!(
                    "unknown directive `{{{{{directive}}}}}`"
                )));
            }
        } else if rest.starts_with('{') {
            let close = rest[1..]
                .find('}')
                .ok_or_else(|| PromptError::ParseError("unclosed variable brace `{`".into()))?;
            let name = rest[1..1 + close].trim();
            if name.is_empty() {
                return Err(PromptError::ParseError("empty variable name `{}`".into()));
            }
            tokens.push(Token::Variable(name.to_string()));
            rest = &rest[1 + close + 1..];
        } else {
            let next = rest.find('{').unwrap_or(rest.len());
            tokens.push(Token::Text(rest[..next].to_string()));
            rest = &rest[next..];
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_plain_text() {
        assert_eq!(parse("hello").unwrap(), vec![Token::Text("hello".into())]);
    }

    #[test]
    fn parses_variable() {
        assert_eq!(
            parse("hi {name}!").unwrap(),
            vec![
                Token::Text("hi ".into()),
                Token::Variable("name".into()),
                Token::Text("!".into()),
            ]
        );
    }

    #[test]
    fn parses_if_block() {
        assert_eq!(
            parse("{{#if x}}yes{{/if}}").unwrap(),
            vec![
                Token::IfStart("x".into()),
                Token::Text("yes".into()),
                Token::IfEnd,
            ]
        );
    }

    #[test]
    fn error_on_unclosed_brace() {
        assert!(parse("hello {name").is_err());
    }

    #[test]
    fn error_on_empty_var() {
        assert!(parse("{}").is_err());
    }
}
