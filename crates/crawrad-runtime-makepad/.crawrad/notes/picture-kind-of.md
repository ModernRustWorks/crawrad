---
id: picture-kind-of
kind: doc
target_file: src/picture.rs
line_hint: 29
author: 
created: 
title: "kind_of"
tags: []
signature: "pub fn kind_of (bytes : & [u8]) -> Option < Kind >"
---

Detects supported image format (PNG or JPEG) from file magic bytes.

The kind of `bytes`, by its signature; `None` for anything else (an SVG or
an icon is not decoded here, and the control stays blank).

# Examples

```rust
use crawrad_runtime_makepad::picture::{kind_of, Kind};

assert_eq!(kind_of(&[0x89, b'P', b'N', b'G']), Some(Kind::Png));
assert_eq!(kind_of(&[0xFF, 0xD8, 0xFF]), Some(Kind::Jpeg));
assert_eq!(kind_of(&[0, 1, 2]), None);
```
