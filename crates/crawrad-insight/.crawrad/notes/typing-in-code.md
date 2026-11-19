---
id: typing-in-code
kind: doc
target_file: src/typing.rs
line_hint: 185
author: 
created: 
title: "in_code"
tags: []
signature: "pub fn in_code (before : & str) -> bool"
---

Checks whether the end of text is outside comments and strings.

Whether the end of `before` is code — not inside a `"…"` string nor
after a `//` comment.

# Examples

```rust
use crawrad_insight::typing::in_code;

assert!(in_code("let x = 1;"));
assert!(!in_code("// comment"));
```
