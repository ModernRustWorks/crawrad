---
id: by-group-with-scheme
kind: doc
target_file: src/lib.rs
line_hint: 6021
author: 
created: 
title: "by_group_with_scheme"
tags: []
signature: "pub fn by_group_with_scheme (group : & str , scheme : KeymapScheme) -> Vec < Action >"
---

Actions belonging to a group in the specified keymap scheme.

# Examples

```rust
use crawrad_actions::{by_group_with_scheme, KeymapScheme};

let file_actions = by_group_with_scheme("File", KeymapScheme::Classic);
assert!(!file_actions.is_empty());
```
