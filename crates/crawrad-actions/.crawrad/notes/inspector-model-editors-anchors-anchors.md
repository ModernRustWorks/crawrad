---
id: inspector-model-editors-anchors-anchors
kind: doc
target_file: src/inspector_model/editors/anchors.rs
line_hint: 40
author: 
created: 
title: "Anchors"
tags: []
signature: "pub struct Anchors"
---

The summary row that folds open into four boxes.

# Examples

```rust
use crawrad_actions::inspector_model::editors::anchors::Anchors;
use crawrad_actions::inspector_model::editor::PropEditor;
use std::collections::BTreeMap;

let anchors = Anchors::new("anchors", BTreeMap::new(), false);
assert_eq!(anchors.template(), "RowFold");
```
