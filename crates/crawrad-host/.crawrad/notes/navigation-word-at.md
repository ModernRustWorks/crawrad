---
id: navigation-word-at
kind: doc
target_file: src/navigation.rs
line_hint: 226
author: 
created: 
title: "word_at"
tags: []
signature: "pub fn word_at (line : & str , byte : usize) -> & str"
---

The name under `byte` of `line` — what "No definition found for" names.

# Examples

```rust
use crawrad_host::navigation::word_at;

assert_eq!(word_at("let my_var = 10;", 6), "my_var");
```
