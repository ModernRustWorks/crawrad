---
id: quick-open-parse-line-column
kind: doc
target_file: src/quick_open.rs
line_hint: 7
author: 
created: 
title: "parse_line_column"
tags: []
signature: "pub fn parse_line_column (input : & str) -> Option < (usize , usize) >"
---

Parses a line and optional column number from user input.

What *Go to Line* typed: `42`, `:42`, `42:10` or `:42:10` (1-based,
never below 1). `None` when it is not a number.

# Examples

```rust
use crawrad_host::quick_open::parse_line_column;

assert_eq!(parse_line_column("42:10"), Some((42, 10)));
assert_eq!(parse_line_column(":15"), Some((15, 1)));
assert_eq!(parse_line_column("invalid"), None);
```
