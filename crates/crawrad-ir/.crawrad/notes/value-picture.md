---
id: value-picture
kind: doc
target_file: src/value.rs
line_hint: 1497
author: 
created: 
title: "Picture"
tags: []
signature: "pub struct Picture"
---

Picture graphic asset in the `.crawform` document.

# Examples

```rust
use crawrad_ir::{ImageFormat, Picture};

let pic = Picture::empty();
assert_eq!(pic.format, ImageFormat::Unknown);
assert!(pic.bytes.is_empty());
```
