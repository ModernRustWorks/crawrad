---
id: list
kind: doc
target_file: src/lib.rs
line_hint: 213
author: 
created: 
title: "list"
tags: []
signature: "pub fn list (tagged : & [Todo]) -> serde_json :: Value"
---

What `todos.list` answers: every tagged comment, 1-based lines.

# Examples

```rust
use crawrad_ext_todo::list;

let json = list(&[]);
assert_eq!(json["count"], 0);
```
