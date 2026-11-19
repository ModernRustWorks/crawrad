---
id: clipboard-clipboard-paste
kind: doc
target_file: src/clipboard.rs
line_hint: 58
author: 
created: 
title: "Clipboard::paste"
tags: []
signature: "pub fn paste (& self , doc : & Document , new_id : impl Fn (& str) -> String ,) -> Option < (Transaction , Vec < NodeId >) >"
---

Pastes copied components into the form.

Pastes into the form. Returns the transaction and the created ids, so the IDE
can select what was just pasted.

# Examples

```rust
use crawrad_designer::clipboard::Clipboard;
use crawrad_ir::{Document, Node, NodeId};

let mut doc = Document::new("form", "Form");
let _ = doc.add_child("form", Node::new("btn", "Button"));
let mut cb = Clipboard::default();
cb.copy(&doc, &[NodeId::from("btn")]);
let res = cb.paste(&doc, |class| format!("{}_copy", class));
assert!(res.is_some());
```
