---
id: targets-makepad-components-edit-default-of
kind: doc
target_file: src/targets/makepad/components/edit.rs
line_hint: 90
author: 
created: 
title: "default_of"
tags: []
signature: "pub fn default_of (node : & Node , desc : & crawrad_registry :: ComponentDesc , text : & str) -> String"
---

Initial state instantiation of an Edit-family text box.

The initial state of a text box of the `Edit` family; `text` is the
expression its `text` field opens with.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::edit::default_of;
use crawrad_ir::Node;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let desc = reg.get("Edit").unwrap();
let node = Node::new("ed", "Edit");
let def = default_of(&node, desc, "\"Hello\".to_string()");
assert!(def.contains("Edit {"));
```
