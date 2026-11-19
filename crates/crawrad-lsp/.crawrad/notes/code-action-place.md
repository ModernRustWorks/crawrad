---
id: code-action-place
kind: doc
target_file: src/code_action.rs
line_hint: 54
author: 
created: 
title: "Place"
tags: []
signature: "pub type Place = (PathBuf , i32 , (usize , usize))"
---

A caret position consisting of file path, document version, line, and byte.

A caret's place: the file, its version, and the line and byte. The byte
counts: an import is offered with the caret ON the name, not at the end
of its line (measured: the same line, two answers).

# Examples

```rust
use crawrad_lsp::code_action::Place;
use std::path::PathBuf;

let place: Place = (PathBuf::from("src/lib.rs"), 1, (10, 4));
assert_eq!(place.1, 1);
```
