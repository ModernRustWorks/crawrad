---
id: catalogue-with-scheme
kind: doc
target_file: src/lib.rs
line_hint: 1569
author: 
created: 
title: "catalogue_with_scheme"
tags: []
signature: "pub fn catalogue_with_scheme (scheme : KeymapScheme) -> Vec < Action >"
---

All actions of the IDE configured with the specified keymap scheme shortcuts.

# Examples

```rust
use crawrad_actions::{catalogue_with_scheme, KeymapScheme};

let actions = catalogue_with_scheme(KeymapScheme::Classic);
assert!(!actions.is_empty());
```
