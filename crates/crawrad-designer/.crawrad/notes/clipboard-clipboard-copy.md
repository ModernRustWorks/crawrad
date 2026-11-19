---
id: clipboard-clipboard-copy
kind: doc
target_file: src/clipboard.rs
line_hint: 48
author: 
created: 
title: "Clipboard::copy"
tags: []
signature: "pub fn copy (& mut self , doc : & Document , sel : & [NodeId]) -> usize"
---

Copies the selection into the clipboard.

Copies the selection. The form does not go along — copying the form
itself does not make sense inside itself.

# Examples

```rust
use crawrad_designer::clipboard::Clipboard;
use crawrad_ir::{Document, Node, NodeId};

let mut doc = Document::new("form", "Form");
let _ = doc.add_child("form", Node::new("btn", "Button"));
let mut cb = Clipboard::default();
let count = cb.copy(&doc, &[NodeId::from("btn")]);
assert_eq!(count, 1);
```
