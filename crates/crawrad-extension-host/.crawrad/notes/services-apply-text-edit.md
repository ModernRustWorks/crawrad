---
id: services-apply-text-edit
kind: doc
target_file: src/services.rs
line_hint: 10
author: 
created: 
title: "apply_text_edit"
tags: []
signature: "pub fn apply_text_edit (content : & str , edit : & TextEdit) -> Result < String , ExtensionError >"
---

Applies a 1-based line/col `TextEdit` onto a string buffer.

Pure helper function to apply a 1-based line/col `TextEdit` onto a string buffer.

# Examples

```rust
use crawrad_extension_api::services::{TextEdit, TextRange};
use crawrad_extension_host::services::apply_text_edit;

let content = "hello world";
let edit = TextEdit::new(TextRange::new(1, 1, 1, 6), "brave");
assert_eq!(apply_text_edit(content, &edit).unwrap(), "brave world");
```

# Errors

Returns [`ExtensionError::ExecutionFailed`](crawrad_extension_api::error::ExtensionError::ExecutionFailed) if lines or columns are out of bounds.
