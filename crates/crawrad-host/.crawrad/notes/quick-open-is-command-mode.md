---
id: quick-open-is-command-mode
kind: doc
target_file: src/quick_open.rs
line_hint: 88
author: 
created: 
title: "is_command_mode"
tags: []
signature: "pub fn is_command_mode (filter : & str) -> bool"
---

A filter that starts with `>` searches commands, not files.

# Examples

```rust
use crawrad_host::quick_open::is_command_mode;

assert!(is_command_mode(">build"));
assert!(!is_command_mode("main.rs"));
```
