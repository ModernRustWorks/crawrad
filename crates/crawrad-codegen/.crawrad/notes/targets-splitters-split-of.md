---
id: targets-splitters-split-of
kind: doc
target_file: src/targets/splitters.rs
line_hint: 103
author: 
created: 
title: "split_of"
tags: []
signature: "pub fn split_of (doc : & Document , field : & str) -> Option < Split >"
---

The Splitter `field`'s neighbours, if it is one.

# Examples

```rust
use crawrad_codegen::targets::splitters::split_of;
use crawrad_ir::Document;

let doc = Document::new("form", "Form");
assert!(split_of(&doc, "split1").is_none());
```
