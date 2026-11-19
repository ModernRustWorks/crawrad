---
id: highlight-highlightanswer
kind: doc
target_file: src/highlight.rs
line_hint: 89
author: 
created: 
title: "HighlightAnswer"
tags: []
signature: "pub struct HighlightAnswer"
---

What the worker answers.

# Examples

```rust
use crawrad_lsp::encoding::PositionEncoding;
use crawrad_lsp::highlight::HighlightAnswer;
use std::path::PathBuf;

let answer = HighlightAnswer {
    file: PathBuf::from("src/lib.rs"),
    version: 1,
    caret: (0, 0),
    highlights: Vec::new(),
    encoding: PositionEncoding::Utf8,
};
assert_eq!(answer.version, 1);
```
