---
id: rename-asked
kind: doc
target_file: src/rename.rs
line_hint: 55
author: 
created: 
title: "Asked"
tags: []
signature: "pub struct Asked < 'a >"
---

Where a rename is asked in editor coordinates.

Where a rename is asked: the file, its version and text, and the caret
(line, byte).

# Examples

```rust
use crawrad_lsp::rename::Asked;
use std::path::Path;

let asked = Asked {
    file: Path::new("src/main.rs"),
    version: 1,
    text: "fn main() {}\n",
    caret: (0, 3),
};
assert_eq!(asked.version, 1);
```
