---
id: targets-makepad-components-codegen-makepadcomponentcodegen-emit-sync
kind: doc
target_file: src/targets/makepad/components/codegen.rs
line_hint: 42
author: 
created: 
title: "MakepadComponentCodegen::emit_sync"
tags: []
signature: "fn emit_sync (& self , doc : & Document , node : & Node , field : & str) -> String"
---

Emits the `sync_ui` synchronization block.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::button::ButtonCodegen;
use crawrad_codegen::targets::makepad::components::codegen::MakepadComponentCodegen;
use crawrad_ir::{Document, Node};

let codegen = ButtonCodegen;
let doc = Document::new("MainForm", "Form");
let node = Node::new("btn", "Button");
let sync = codegen.emit_sync(&doc, &node, "btn");
assert!(sync.contains("btn.set_text"));
```
