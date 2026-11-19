---
id: value-shortcut
kind: doc
target_file: src/value.rs
line_hint: 1186
author: 
created: 
title: "Shortcut"
tags: []
signature: "pub struct Shortcut"
---

Keyboard shortcut combining modifiers with a primary key.

Keyboard shortcut combining modifiers with a primary key in the `.crawform` document.

# Examples

```rust
use crawrad_ir::{Key, Shortcut};

let sc = Shortcut::new(Key::A, true, false, false);
assert!(sc.ctrl);
assert!(!sc.alt);
```
