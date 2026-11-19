---
id: document-file-label
kind: doc
target_file: src/document.rs
line_hint: 143
author: 
created: 
title: "file_label"
tags: []
signature: "pub fn file_label (aberto : Option < & Path > , falha : Option < & str >) -> String"
---

What the status bar says on the LEFT: the open file, or why there is
not one.

# Examples

```rust
use crawrad_host::document::file_label;

assert_eq!(file_label(None, None), "no form open");
```
