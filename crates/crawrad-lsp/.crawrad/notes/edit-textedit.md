---
id: edit-textedit
kind: doc
target_file: src/edit.rs
line_hint: 15
author: 
created: 
title: "TextEdit"
tags: []
signature: "pub struct TextEdit"
---

A text replacement where `range` becomes `new_text`.

One replacement: `range` (UTF-16 columns, as the LSP counts) becomes
`new_text`.

# Examples

```rust
use crawrad_lsp::edit::TextEdit;
use crawrad_lsp::protocol::{Position, Range};

let edit = TextEdit {
    range: Range::new(Position::new(0, 0), Position::new(0, 4)),
    new_text: "let".to_string(),
};
assert_eq!(edit.new_text, "let");
```
