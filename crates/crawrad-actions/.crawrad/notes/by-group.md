---
id: by-group
kind: doc
target_file: src/lib.rs
line_hint: 6016
author: 
created: 
title: "by_group"
tags: []
signature: "pub fn by_group (group : & str) -> Vec < Action >"
---

Actions belonging to a group in the default VS Code keymap scheme.

# Examples

```rust
use crawrad_actions::by_group;

let file_actions = by_group("File");
assert!(!file_actions.is_empty());
```
