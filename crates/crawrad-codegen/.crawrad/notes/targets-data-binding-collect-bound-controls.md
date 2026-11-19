---
id: targets-data-binding-collect-bound-controls
kind: doc
target_file: src/targets/data_binding.rs
line_hint: 144
author: 
created: 
title: "collect_bound_controls"
tags: []
signature: "pub fn collect_bound_controls (doc : & Document , emitted : & [& Node] ,) -> Result < Vec < BoundControl > , Erro >"
---

Collects valid scalar bindings in emitted nodes within the beta scope.

Collects all valid scalar bindings in `emitted` nodes within the beta scope
of #788.

# Beta scope (#788)

Only two scalar properties are bound: `Edit.text` and `CheckBox.checked`, each to a
column of a `Dataset` component connected (by `Value::Ref`) to a `Connection`.
Any other property or class, a missing column, a source that is not a `Dataset`
or a dangling reference emits nothing and allocates no adapter state.

A bound edit puts the dataset in Edit; nothing here posts it -- the dataset's
navigation and Post/Cancel are its standard actions (`collect_dataset_actions`).

# Examples

```rust
use crawrad_codegen::targets::data_binding::collect_bound_controls;
use crawrad_ir::Document;

let doc = Document::new("MainForm", "Form");
let controls = collect_bound_controls(&doc, &[]).unwrap();
assert!(controls.is_empty());
```

# Errors

Returns an error if any bound control has an invalid identifier.
