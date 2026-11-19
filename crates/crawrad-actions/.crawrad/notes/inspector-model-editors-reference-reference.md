---
id: inspector-model-editors-reference-reference
kind: doc
target_file: src/inspector_model/editors/reference.rs
line_hint: 31
author: 
created: 
title: "Reference"
tags: []
signature: "pub struct Reference"
---

A drop-down over the components of one class.

# Examples

```rust
use crawrad_actions::inspector_model::editors::reference::Reference;
use crawrad_actions::inspector_model::editor::PropEditor;

let r = Reference::new(vec![], "");
assert_eq!(r.template(), "RowChoice");
```
