---
id: inspector-model-editors-reference-none
kind: doc
target_file: src/inspector_model/editors/reference.rs
line_hint: 26
author: 
created: 
title: "NONE"
tags: []
signature: "pub const NONE : & str = crawrad_ir :: value :: NO_REFERENCE"
---

What an empty reference reads as.

The same word the value summary uses.

# Examples

```rust
use crawrad_actions::inspector_model::editors::reference::NONE;

assert_eq!(NONE, "(none)");
```
