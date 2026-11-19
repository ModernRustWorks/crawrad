---
id: definition-target
kind: doc
target_file: src/definition.rs
line_hint: 21
author: 
created: 
title: "Target"
tags: []
signature: "pub struct Target"
---

A place a definition lives: file, line and byte (0-based).

# Examples

```rust
use crawrad_lsp::definition::Target;
use std::path::PathBuf;

let target = Target { file: PathBuf::from("src/lib.rs"), line: 10, byte: 4 };
assert_eq!(target.line, 10);
```
