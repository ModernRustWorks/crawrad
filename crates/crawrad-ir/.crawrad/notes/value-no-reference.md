---
id: value-no-reference
kind: doc
target_file: src/value.rs
line_hint: 40
author: 
created: 
title: "NO_REFERENCE"
tags: []
signature: "pub const NO_REFERENCE : & str = \"(none)\""
---

What an empty reference reads as.

# Examples

```rust
use crawrad_ir::value::NO_REFERENCE;

assert_eq!(NO_REFERENCE, "(none)");
```
