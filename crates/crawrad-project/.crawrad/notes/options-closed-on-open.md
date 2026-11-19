---
id: options-closed-on-open
kind: doc
target_file: src/options.rs
line_hint: 434
author: 
created: 
title: "closed_on_open"
tags: []
signature: "pub fn closed_on_open () -> Vec < String >"
---

Groups that start CLOSED, to seed the folded set.

# Examples

```rust
use crawrad_project::options::closed_on_open;

let closed = closed_on_open();
assert!(closed.contains(&"Keyboard".to_string()));
```
