---
id: inspector-model-editors-collection-collection
kind: doc
target_file: src/inspector_model/editors/collection.rs
line_hint: 102
author: 
created: 
title: "Collection"
tags: []
signature: "pub struct Collection"
---

A button that opens the text pad, and the count beside it.

# Examples

```rust
use crawrad_actions::inspector_model::editors::collection::Collection;
use crawrad_actions::inspector_model::editor::PropEditor;

let col = Collection::new("tree".to_string(), "items", vec![], false);
assert_eq!(col.template(), "RowOpen");
```
