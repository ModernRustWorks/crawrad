---
id: targets-makepad-components-emit-sync
kind: doc
target_file: src/targets/makepad/components/mod.rs
line_hint: 321
author: 
created: 
title: "emit_sync"
tags: []
signature: "pub fn emit_sync (doc : & Document , node : & Node , field : & str) -> String"
---

Emits UI synchronization logic for `sync_ui`.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::emit_sync;
use crawrad_ir::{Document, Node};

let doc = Document::new("MainForm", "Form");
let node = Node::new("btn", "Button");
let sync = emit_sync(&doc, &node, "btn");
assert!(sync.contains("btn"));
```
