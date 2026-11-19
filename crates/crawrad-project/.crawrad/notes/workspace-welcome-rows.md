---
id: workspace-welcome-rows
kind: doc
target_file: src/workspace.rs
line_hint: 381
author: 
created: 
title: "welcome_rows"
tags: []
signature: "pub fn welcome_rows () -> Vec < WelcomeRow >"
---

The Welcome page's Recent list, read from disk now.

# Examples

```rust
use crawrad_project::workspace::welcome_rows;

let rows = welcome_rows();
assert!(rows.len() <= 10);
```
