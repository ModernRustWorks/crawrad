---
id: signature-asked
kind: doc
target_file: src/signature.rs
line_hint: 117
author: 
created: 
title: "Asked"
tags: []
signature: "pub struct Asked < 'a >"
---

One request: where, and why.

# Examples

```rust
use crawrad_lsp::signature::{Asked, Trigger};
use std::path::Path;

let asked = Asked {
    file: Path::new("src/main.rs"),
    version: 1,
    text: "foo()\n",
    caret: (0, 4),
    trigger: Trigger::Invoked,
    retrigger: false,
};
assert_eq!(asked.version, 1);
```
