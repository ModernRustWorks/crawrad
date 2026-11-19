---
id: is-known-action
kind: doc
target_file: src/lib.rs
line_hint: 1029
author: 
created: 
title: "is_known_action"
tags: []
signature: "pub fn is_known_action (id : & str) -> bool"
---

Returns true if `id` is a known action identifier in the catalogue.

# Examples

```rust
use crawrad_actions::is_known_action;

assert!(is_known_action("file.save"));
assert!(!is_known_action("nonexistent.action"));
```
