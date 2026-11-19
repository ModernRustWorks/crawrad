---
id: literal
kind: doc
target_file: src/lib.rs
line_hint: 858
author: 
created: 
title: "literal"
tags: []
signature: "pub fn literal (s : & str) -> String"
---

An `&str` as a Rust literal, with anything needing escaping escaped.

# Examples

```rust
use crawrad_codegen::literal;

assert_eq!(literal("hello\n\"world\""), "\"hello\\n\\\"world\\\"\"");
```
