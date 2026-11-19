---
id: encoding-lineending-detect
kind: doc
target_file: src/encoding.rs
line_hint: 39
author: 
created: 
title: "LineEnding::detect"
tags: []
signature: "pub fn detect (raw : & str) -> Self"
---

Detects the line ending convention used in the raw string.

**The first line ending the file shows decides for the whole file.**

A file with MIXED endings is already inconsistent, and the IDE is not a
normalisation tool: it picks one and says nothing. What it must not do
is the opposite — change the ending of a whole file without being
asked.

# Examples

```rust
use crawrad_project::encoding::LineEnding;

assert_eq!(LineEnding::detect("a\r\nb"), LineEnding::CrLf);
assert_eq!(LineEnding::detect("a\nb"), LineEnding::Lf);
```
