---
id: session-shelltype
kind: doc
target_file: src/session.rs
line_hint: 2
author: 
created: 
title: "ShellType"
tags: []
signature: "pub enum ShellType"
---

Available shell environments in the integrated terminal.

# Examples

```rust
use crawrad_terminal::ShellType;

assert_eq!(ShellType::Cmd.display_name(), "cmd");
```
