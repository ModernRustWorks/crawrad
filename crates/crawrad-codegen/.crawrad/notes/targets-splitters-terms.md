---
id: targets-splitters-terms
kind: doc
target_file: src/targets/splitters.rs
line_hint: 79
author: 
created: 
title: "terms"
tags: []
signature: "pub fn terms (doc : & Document , node : & Node , prefix : & str) -> [String ; 4]"
---

Returns splitter geometry offset terms for a node's x, y, w, and h.

What `node` adds to its literal x, y, w and h: `prefix` is how the
generated code reaches the form's state (`self.` on egui, `self.form.`
on Makepad).

# Examples

```rust
use crawrad_codegen::targets::splitters::terms;
use crawrad_ir::{Document, Node};

let doc = Document::new("form", "Form");
let node = Node::new("btn", "Button");
let t = terms(&doc, &node, "self.");
assert_eq!(t[0], "");
```
