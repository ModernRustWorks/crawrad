---
id: encoding-encoding-of
kind: doc
target_file: src/encoding.rs
line_hint: 75
author: 
created: 
title: "Encoding::of"
tags: []
signature: "pub fn of (bytes : & [u8]) -> Self"
---

Reads both facts off the raw bytes of a file.

Bytes and not a `String`, and that is the whole reason this takes the
raw slice: a BOM decoded into a `String` becomes an invisible
`U+FEFF` at the front, and from then on nobody can tell it from a
character the user typed.

# Examples

```rust
use crawrad_project::encoding::{Encoding, LineEnding};

let enc = Encoding::of(b"hello\nworld");
assert!(!enc.bom);
assert_eq!(enc.line_ending, LineEnding::Lf);
```
