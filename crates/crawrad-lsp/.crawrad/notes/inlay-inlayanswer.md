---
id: inlay-inlayanswer
kind: doc
target_file: src/inlay.rs
line_hint: 100
author: 
created: 
title: "InlayAnswer"
tags: []
signature: "pub struct InlayAnswer"
---

What the worker answers: the file, the version it was asked for, the hints.

# Examples

```rust
use crawrad_lsp::encoding::PositionEncoding;
use crawrad_lsp::inlay::InlayAnswer;
use std::path::PathBuf;

let answer = InlayAnswer {
    file: PathBuf::from("src/lib.rs"),
    version: 1,
    hints: Vec::new(),
    encoding: PositionEncoding::Utf8,
};
assert_eq!(answer.version, 1);
```
