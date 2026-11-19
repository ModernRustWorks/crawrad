---
id: targets-data-binding-dataset-action-of
kind: doc
target_file: src/targets/data_binding.rs
line_hint: 48
author: 
created: 
title: "dataset_action_of"
tags: []
signature: "pub fn dataset_action_of (doc : & Document , act_node : & Node) -> Result < Option < DatasetAction > , Erro >"
---

Resolves an `Action` node to a `DatasetAction` if bound to a dataset operation.

Resolves a single `Action` node to a `DatasetAction` if it binds to a valid
dataset operation.

# Examples

```rust
use crawrad_codegen::targets::data_binding::dataset_action_of;
use crawrad_ir::{Document, Node};

let doc = Document::new("MainForm", "Form");
let node = Node::new("btn", "Button");
assert_eq!(dataset_action_of(&doc, &node).unwrap(), None);
```

# Errors

Returns an error if the action node name or dataset references are invalid.
