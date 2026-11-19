---
id: value-filepath
kind: doc
target_file: src/value.rs
line_hint: 363
author: 
created: 
title: "FilePath"
tags: []
signature: "pub struct FilePath"
---

File path in the `.crawform` document.

# Examples

```rust
use crawrad_ir::FilePath;

let fp = FilePath::new("src/main.rs");
assert_eq!(fp.path, "src/main.rs");
```
