---
id: same-file
kind: doc
target_file: src/lib.rs
line_hint: 141
author: 
created: 
title: "same_file"
tags: []
signature: "pub fn same_file (a : & str , b : & str) -> bool"
---

The two texts are the same file, ignoring the checkout's line breaks.

# Examples

```rust
use crawrad_cli::same_file;

assert!(same_file("hello\r\n", "hello\n"));
```
