---
id: prompt-assembler-projectcontext-language-of
kind: doc
target_file: src/prompt_assembler.rs
line_hint: 83
author: 
created: 
title: "ProjectContext::language_of"
tags: []
signature: "pub fn language_of (path : & Path) -> String"
---

Determines the language name for a file based on its extension.

The language a file's extension names for the agent: `rs` is `rust`,
`md` is `markdown`, any other extension is itself, none is `text`.

# Examples

```rust
use crawrad_agent::ProjectContext;
use std::path::Path;

assert_eq!(ProjectContext::language_of(Path::new("src/main.rs")), "rust");
```
