---
id: typing-dot-context
kind: doc
target_file: src/typing.rs
line_hint: 17
author: 
created: 
title: "dot_context"
tags: []
signature: "pub fn dot_context (line : & str , byte : usize) -> (Option < String > , String , usize)"
---

Extracts dot completion context including base, prefix, and replace offset.

What is before the caret: the `base` of `base.pre|` (when there is a dot),
the prefix typed, and the byte where the completion will replace from.

# Examples

```rust
use crawrad_insight::typing::dot_context;

let (base, prefix, replace_from) = dot_context("self.len", 8);
assert_eq!(base.as_deref(), Some("self"));
assert_eq!(prefix, "len");
```
