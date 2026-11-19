---
id: inspector-model-editors-edges-edges
kind: doc
target_file: src/inspector_model/editors/edges.rs
line_hint: 65
author: 
created: 
title: "Edges"
tags: []
signature: "pub struct Edges"
---

The summary row that folds open into four.

# Examples

```rust
use crawrad_actions::inspector_model::editors::edges::Edges;
use crawrad_actions::inspector_model::editor::PropEditor;
use crawrad_ir::Edges as IrEdges;

let ed = Edges::new("margins", IrEdges::uniform(0.0), false);
assert_eq!(ed.template(), "RowFold");
```
