---
id: action
kind: doc
target_file: src/lib.rs
line_hint: 1197
author: 
created: 
title: "Action"
tags: []
signature: "pub struct Action"
---

An action in the catalogue.

# Examples

```rust
use crawrad_actions::by_id;

let action = by_id("file.save").unwrap();
assert_eq!(action.id, "file.save");
```
