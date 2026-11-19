---
id: workspace-decide
kind: doc
target_file: src/workspace.rs
line_hint: 225
author: 
created: 
title: "decide"
tags: []
signature: "pub fn decide (documents : Vec < PathBuf >) -> OnOpen"
---

Decides what to do with the documents found in a root.

The decision, separated from directory scanning to enable testing without touching the disk.

# Examples

```rust
use crawrad_project::workspace::{decide, OnOpen};

assert_eq!(decide(Vec::new()), OnOpen::NoForm);
```
