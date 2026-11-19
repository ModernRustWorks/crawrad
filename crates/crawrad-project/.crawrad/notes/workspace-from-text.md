---
id: workspace-from-text
kind: doc
target_file: src/workspace.rs
line_hint: 275
author: 
created: 
title: "from_text"
tags: []
signature: "pub fn from_text (text : & str) -> Vec < PathBuf >"
---

The file text becomes a list.

# Examples

```rust
use std::path::PathBuf;
use crawrad_project::workspace::from_text;

let list = from_text("foo\nbar\n");
assert_eq!(list, vec![PathBuf::from("foo"), PathBuf::from("bar")]);
```
