---
id: typing-word-prefix
kind: doc
target_file: src/typing.rs
line_hint: 10
author: 
created: 
title: "word_prefix"
tags: []
signature: "pub fn word_prefix (line : & str , byte : usize) -> & str"
---

The name being typed just before `byte` in `line`: `St` in `let x = St`.

# Examples

```rust
use crawrad_insight::typing::word_prefix;

assert_eq!(word_prefix("let x = St", 10), "St");
```
