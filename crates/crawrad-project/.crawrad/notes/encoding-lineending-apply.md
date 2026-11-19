---
id: encoding-lineending-apply
kind: doc
target_file: src/encoding.rs
line_hint: 48
author: 
created: 
title: "LineEnding::apply"
tags: []
signature: "pub fn apply (self , text : & str) -> std :: borrow :: Cow < '_ , str >"
---

The text ready for disk.

# Examples

```rust
use crawrad_project::encoding::LineEnding;

assert_eq!(LineEnding::CrLf.apply("a\nb"), "a\r\nb");
```
