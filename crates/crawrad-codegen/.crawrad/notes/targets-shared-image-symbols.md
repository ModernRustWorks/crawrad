---
id: targets-shared-image-symbols
kind: doc
target_file: src/targets/shared.rs
line_hint: 520
author: 
created: 
title: "image_symbols"
tags: []
signature: "pub fn image_symbols (doc : & crawrad_ir :: Document , node : & crawrad_ir :: Node) -> Vec < String >"
---

The icon symbols of the `ImageList` linked by `node`'s `images`.

# Examples

```rust
use crawrad_codegen::targets::shared::image_symbols;
use crawrad_ir::{Document, Node};

let doc = Document::new("form", "Form");
let node = Node::new("tree", "TreeView");
let syms = image_symbols(&doc, &node);
assert!(syms.is_empty());
```
