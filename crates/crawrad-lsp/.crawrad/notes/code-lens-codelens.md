---
id: code-lens-codelens
kind: doc
target_file: src/code_lens.rs
line_hint: 22
author: 
created: 
title: "CodeLens"
tags: []
signature: "pub struct CodeLens"
---

One lens as LSP 3.17 sends it.

# Examples

```rust
use crawrad_lsp::code_lens::CodeLens;
use crawrad_lsp::protocol::{Position, Range};

let range = Range::new(Position::new(0, 0), Position::new(0, 5));
let lens = CodeLens::new(range, None);
assert!(lens.command.is_none());
```
