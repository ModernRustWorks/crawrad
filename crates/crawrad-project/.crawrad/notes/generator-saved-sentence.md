---
id: generator-saved-sentence
kind: doc
target_file: src/generator.rs
line_hint: 361
author: 
created: 
title: "saved_sentence"
tags: []
signature: "pub fn saved_sentence (path : & Path) -> String"
---

What the bar says when `Generate` saved the editor on its own.

**In English, and naming the file.** Saving without being asked is the kind
of thing that frightens the first time and pleases the second — and what
decides between the two is the IDE saying what it did.

# Examples

```rust
use std::path::Path;
use crawrad_project::generator::saved_sentence;

assert_eq!(saved_sentence(Path::new("src/main.rs")), "saved main.rs");
```
