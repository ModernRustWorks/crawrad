---
id: workspace-to-text
kind: doc
target_file: src/workspace.rs
line_hint: 291
author: 
created: 
title: "to_text"
tags: []
signature: "pub fn to_text (list : & [PathBuf]) -> String"
---

The list becomes the file text.

# Examples

```rust
use std::path::PathBuf;
use crawrad_project::workspace::to_text;

let text = to_text(&[PathBuf::from("foo")]);
assert!(text.contains("foo"));
```
