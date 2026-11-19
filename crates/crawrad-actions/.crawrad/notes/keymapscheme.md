---
id: keymapscheme
kind: doc
target_file: src/lib.rs
line_hint: 1034
author: 
created: 
title: "KeymapScheme"
tags: []
signature: "pub enum KeymapScheme"
---

Keyboard shortcut schemes supported by the IDE.

# Examples

```rust
use crawrad_actions::KeymapScheme;

let scheme = KeymapScheme::Modern;
assert_eq!(scheme.id(), "modern");
```
