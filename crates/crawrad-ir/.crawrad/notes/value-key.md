---
id: value-key
kind: doc
target_file: src/value.rs
line_hint: 885
author: 
created: 
title: "Key"
tags: []
signature: "pub enum Key"
---

Primary key for keyboard shortcuts in the `.crawform` document.

# Examples

```rust
use crawrad_ir::Key;

assert_eq!(Key::default(), Key::Unknown);
assert_eq!(Key::Escape.as_str(), "Esc");
```
