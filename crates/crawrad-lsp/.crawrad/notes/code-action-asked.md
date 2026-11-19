---
id: code-action-asked
kind: doc
target_file: src/code_action.rs
line_hint: 130
author: 
created: 
title: "Asked"
tags: []
signature: "pub struct Asked < 'a >"
---

One request: where.

# Examples

```rust
use crawrad_lsp::code_action::Asked;
use std::path::Path;

let asked = Asked {
    file: Path::new("src/lib.rs"),
    version: 1,
    text: "fn main() {}",
    caret: (0, 0),
};
assert_eq!(asked.version, 1);
```
