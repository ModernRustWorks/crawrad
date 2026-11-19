---
id: by-id
kind: doc
target_file: src/lib.rs
line_hint: 6004
author: 
created: 
title: "by_id"
tags: []
signature: "pub fn by_id (id : & str) -> Option < Action >"
---

Finds an action by id in the default VS Code keymap scheme.

# Examples

```rust
use crawrad_actions::by_id;

let action = by_id("file.save");
assert!(action.is_some());
```
