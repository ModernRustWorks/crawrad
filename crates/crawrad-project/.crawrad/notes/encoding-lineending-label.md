---
id: encoding-lineending-label
kind: doc
target_file: src/encoding.rs
line_hint: 56
author: 
created: 
title: "LineEnding::label"
tags: []
signature: "pub fn label (self) -> & 'static str"
---

What the footer calls it.

# Examples

```rust
use crawrad_project::encoding::LineEnding;

assert_eq!(LineEnding::Lf.label(), "LF");
assert_eq!(LineEnding::CrLf.label(), "CRLF");
```
