---
id: targets-splitters-splits
kind: doc
target_file: src/targets/splitters.rs
line_hint: 31
author: 
created: 
title: "splits"
tags: []
signature: "pub fn splits (doc : & Document) -> Vec < Split >"
---

Every Splitter of `doc`, with its neighbours.

# Examples

```rust
use crawrad_codegen::targets::splitters::splits;
use crawrad_ir::Document;

let doc = Document::new("form", "Form");
let list = splits(&doc);
assert!(list.is_empty());
```
