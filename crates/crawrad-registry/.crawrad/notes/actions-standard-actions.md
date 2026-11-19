---
id: actions-standard-actions
kind: doc
target_file: src/actions.rs
line_hint: 41
author: 
created: 
title: "standard_actions"
tags: []
signature: "pub fn standard_actions () -> Vec < StandardAction >"
---

All standard-action templates known to the IDE.

# Examples

```rust
use crawrad_registry::standard_actions;

let actions = standard_actions();
assert!(!actions.is_empty());
```
