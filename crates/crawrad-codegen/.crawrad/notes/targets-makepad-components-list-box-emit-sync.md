---
id: targets-makepad-components-list-box-emit-sync
kind: doc
target_file: src/targets/makepad/components/list_box.rs
line_hint: 115
author: 
created: 
title: "emit_sync"
tags: []
signature: "pub fn emit_sync (_doc : & Document , _node : & Node , field : & str) -> String"
---

Synchronizes the form's items and selection with the CrawradListBox widget.

#439: the form's items and selection, at run time, handed to the
`CrawradListBox` (it only redraws when they change).

# Examples

```rust
use crawrad_codegen::targets::makepad::components::list_box::emit_sync;
use crawrad_ir::{Document, Node};

let doc = Document::new("MainForm", "Form");
let node = Node::new("lb", "ListBox");
let sync = emit_sync(&doc, &node, "lb");
assert!(sync.contains("lb"));
```
