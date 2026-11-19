---
id: by-id-with-scheme
kind: doc
target_file: src/lib.rs
line_hint: 6009
author: 
created: 
title: "by_id_with_scheme"
tags: []
signature: "pub fn by_id_with_scheme (id : & str , scheme : KeymapScheme) -> Option < Action >"
---

Finds an action by id in the specified keymap scheme.

# Examples

```rust
use crawrad_actions::{by_id_with_scheme, KeymapScheme};

let action = by_id_with_scheme("file.save", KeymapScheme::Classic);
assert!(action.is_some());
```
