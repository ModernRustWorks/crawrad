---
id: drawing-ink-from-name
kind: doc
target_file: src/drawing.rs
line_hint: 52
author: 
created: 
title: "Ink::from_name"
tags: []
signature: "pub fn from_name (s : & str) -> Option < Self >"
---

Parses an ink role from a manifest name.

Read from the manifest. `None` for a name that is no role at all — and the gate
rejects it, instead of picking a color on its own.

# Examples

```rust
use crawrad_ir::drawing::Ink;

assert_eq!(Ink::from_name("accent"), Some(Ink::Accent));
assert_eq!(Ink::from_name("invalid"), None);
```
