---
id: no-canvas
kind: doc
target_file: src/lib.rs
line_hint: 618
author: 
created: 
title: "no_canvas"
tags: []
signature: "pub fn no_canvas (doc : & Document , id : & str) -> bool"
---

Checks if a node appears on the design canvas.

# Examples

```rust
use crawrad_ir::Document;
use crawrad_runtime_makepad::no_canvas;

let doc = Document::new("Form1", "TForm");
assert!(no_canvas(&doc, &doc.root));
```
