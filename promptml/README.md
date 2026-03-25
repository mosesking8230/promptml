# promptml

Type-safe LLM prompt templates for Rust, with compile-time variable checking.

[![Crates.io](https://img.shields.io/crates/v/promptml.svg)](https://crates.io/crates/promptml)
[![Docs.rs](https://docs.rs/promptml/badge.svg)](https://docs.rs/promptml)
[![CI](https://github.com/ptcodes/promptml/actions/workflows/ci.yml/badge.svg)](https://github.com/ptcodes/promptml/actions/workflows/ci.yml)

## Why

Prompt templates in most LLM libraries are plain strings. Forget a variable
and you get a broken prompt at runtime — or worse, silently wrong output.
`promptml` catches missing variables at **compile time** using a proc macro,
and at runtime with clear, descriptive error messages.

## Install

```toml
[dependencies]
promptml = "0.1"
```

## Usage

### Compile-time safety with `prompt!`

```rust,no_run
use promptml::prompt;

let t = prompt!("Translate {text} to {language}.");

// Compiles and runs fine
let out = t.render()
    .set("text", "Hello")
    .set("language", "Spanish")
    .build()?;

// Fails at compile time — missing variable caught before you ship
let bad = t.render()
    .set("text", "Hello")
    .build();
# Ok::<(), promptml::PromptError>(())
```

### Runtime templates

```rust,no_run
use promptml::PromptTemplate;

let t = PromptTemplate::new("Summarise {topic} in {n_words} words.")?;
let out = t.render()
    .set("topic", "Rust lifetimes")
    .set("n_words", "50")
    .build()?;
# Ok::<(), promptml::PromptError>(())
```

### Optional blocks

```rust,no_run
let t = promptml::prompt!(
    "Answer: {question}{{#if context}}\nContext: {context}{{/if}}"
);
```

### Few-shot examples

```rust,no_run
use promptml::{prompt, Example};
use std::collections::HashMap;

let t = prompt!("{{#examples}}{input} → {label}\n{{/examples}}\nClassify: {input}");
let mut e1 = HashMap::new();
e1.insert("input".to_string(), "Great!".to_string());
e1.insert("label".to_string(), "positive".to_string());
let out = t.render()
    .set("input", "I love this!")
    .examples(vec![Example { vars: e1 }])
    .build()?;
# Ok::<(), promptml::PromptError>(())
```

### Chat message format

```rust,no_run
use promptml::chat_prompt;

let messages = chat_prompt! {
    system: "You are a {persona}.",
    user: "{question}",
}
.render()
.set("persona", "Rust expert")
.set("question", "What is a lifetime?")
.to_messages()?;
# Ok::<(), promptml::PromptError>(())
```

### Load templates from a TOML file

```rust,no_run
use promptml::PromptTemplate;

let t = PromptTemplate::from_file("prompts/summarise.toml")?;
# Ok::<(), promptml::PromptError>(())
```

```toml
# prompts/summarise.toml
[template]
name = "summarise"

[messages]
system = "You are a concise technical writer."
user   = "Summarise the following {doc_type} in {max_bullets} bullet points.\n\n{content}"
```

## License

MIT — see [LICENSE-MIT](../LICENSE-MIT)
