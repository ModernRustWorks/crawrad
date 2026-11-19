---
id: inspector-model-editors-rect-rectrow
kind: doc
target_file: src/inspector_model/editors/rect.rs
line_hint: 71
author: 
created: 
title: "RectRow"
tags: []
signature: "pub struct RectRow"
---

The summary row that folds open into four.

# Examples

```rust
use crawrad_actions::inspector_model::editors::rect::RectRow;
use crawrad_actions::inspector_model::editor::PropEditor;
use crawrad_ir::Rect;

let row = RectRow::new("rect", Rect::new(0.0, 0.0, 100.0, 50.0), false);
assert_eq!(row.template(), "RowFold");
```
