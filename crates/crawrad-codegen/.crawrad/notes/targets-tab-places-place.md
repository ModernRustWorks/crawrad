---
id: targets-tab-places-place
kind: doc
target_file: src/targets/tab_places.rs
line_hint: 22
author: 
created: 
title: "place"
tags: []
signature: "pub fn place (reg : & crawrad_registry :: Registry , doc : & Document , n : & Node) -> Option < Vec < i64 > >"
---

Computes a control's hierarchical place in the Tab sequence.

`n`'s place in the Tab sequence, `None` when it takes no part: its class
cannot take the focus (it declares no `OnEnter`), has no `TabStop` property
(#139), has no `TabOrder`, or the user turned its `TabStop` off.

# Examples

```rust
use crawrad_codegen::targets::tab_places::place;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let reg = Registry::builtin();
let doc = Document::new("form", "Form");
let node = Node::new("btn", "Button");
let p = place(&reg, &doc, &node);
let _ = p;
```
