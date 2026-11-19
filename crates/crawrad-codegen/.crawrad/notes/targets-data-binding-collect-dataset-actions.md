---
id: targets-data-binding-collect-dataset-actions
kind: doc
target_file: src/targets/data_binding.rs
line_hint: 90
author: 
created: 
title: "collect_dataset_actions"
tags: []
signature: "pub fn collect_dataset_actions (doc : & Document , emitted : & [& Node] ,) -> Result < Vec < DatasetAction > , Erro >"
---

Collects all valid dataset actions in `emitted` nodes.

# Examples

```rust
use crawrad_codegen::targets::data_binding::collect_dataset_actions;
use crawrad_ir::Document;

let doc = Document::new("MainForm", "Form");
let actions = collect_dataset_actions(&doc, &[]).unwrap();
assert!(actions.is_empty());
```

# Errors

Returns an error if action names or dataset references are invalid.
