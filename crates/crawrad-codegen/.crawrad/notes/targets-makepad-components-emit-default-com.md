---
id: targets-makepad-components-emit-default-com
kind: doc
target_file: src/targets/makepad/components/mod.rs
line_hint: 235
author: 
created: 
title: "emit_default_com"
tags: []
signature: "pub fn emit_default_com (reg : & crawrad_registry :: Registry , doc : & crawrad_ir :: Document , node : & Node ,) -> String"
---

Emits the default initialization of a node using the component catalog.

The same, with the CATALOG in hand — and this is the one the form calls.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::emit_default_com;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let reg = Registry::builtin();
let doc = Document::new("MainForm", "Form");
let node = Node::new("btn", "Button");
let def = emit_default_com(&reg, &doc, &node);
assert!(def.contains("Button {"));
```
