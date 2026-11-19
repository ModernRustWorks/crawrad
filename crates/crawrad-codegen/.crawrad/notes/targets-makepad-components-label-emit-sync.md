---
id: targets-makepad-components-label-emit-sync
kind: doc
target_file: src/targets/makepad/components/label.rs
line_hint: 79
author: 
created: 
title: "emit_sync"
tags: []
signature: "pub fn emit_sync (_doc : & Document , node : & Node , field : & str) -> String"
---

The text goes to the `Label` INSIDE the wrapper — see `emit_dsl`.

The `id!` descends one level intentionally: the instance name belongs to the `View` that
positions, because visibility and clipping pass through it.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::label::emit_sync;
use crawrad_ir::{Document, Node};

let doc = Document::new("MainForm", "Form");
let node = Node::new("lbl", "Label");
let sync = emit_sync(&doc, &node, "lbl");
assert!(sync.contains("lbl"));
```
