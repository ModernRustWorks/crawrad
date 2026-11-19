---
id: catalogue
kind: doc
target_file: src/lib.rs
line_hint: 1564
author: 
created: 
title: "catalogue"
tags: []
signature: "pub fn catalogue () -> Vec < Action >"
---

All actions of the IDE in the default Modern keymap scheme.

# Examples

```rust
use crawrad_actions::catalogue;

let actions = catalogue();
assert!(!actions.is_empty());
```
