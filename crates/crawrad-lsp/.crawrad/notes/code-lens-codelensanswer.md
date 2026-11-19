---
id: code-lens-codelensanswer
kind: doc
target_file: src/code_lens.rs
line_hint: 180
author: 
created: 
title: "CodeLensAnswer"
tags: []
signature: "pub struct CodeLensAnswer"
---

What the worker answers: the file, the version it was asked for, the lenses.

# Examples

```rust
use crawrad_lsp::code_lens::CodeLensAnswer;
use std::path::PathBuf;

let answer = CodeLensAnswer {
    file: PathBuf::from("src/lib.rs"),
    version: 1,
    lenses: Vec::new(),
};
assert_eq!(answer.version, 1);
```
