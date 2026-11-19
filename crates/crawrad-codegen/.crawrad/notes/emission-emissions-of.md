---
id: emission-emissions-of
kind: doc
target_file: src/emission.rs
line_hint: 128
author: 
created: 
title: "emissions_of"
tags: []
signature: "pub fn emissions_of (doc : & Document , node : & Node , desc : & ComponentDesc) -> Vec < Emission >"
---

**The verbs of a node** — generic, without a single line of class knowledge.

The order matters and follows ADR 0036: `Create` first, `Set` next, `Child`
and `Bind` last. Any code reading a reference (`Value::Ref`) already finds the
target created.

The value of each `Set` is **that of the node when it has one, and the descriptor's default
when it does not** — which ensures that an older document, saved before the property
existed, emits the same as a new one.

# Examples

```rust
use crawrad_codegen::emission::emissions_of;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let doc = Document::new("MainForm", "Form");
let node = Node::new("button1", "Button");
let reg = Registry::builtin();
let desc = reg.get("Button").unwrap();
let emissions = emissions_of(&doc, &node, desc);
assert!(!emissions.is_empty());
```
