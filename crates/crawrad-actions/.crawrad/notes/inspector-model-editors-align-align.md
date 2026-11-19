---
id: inspector-model-editors-align-align
kind: doc
target_file: src/inspector_model/editors/align.rs
line_hint: 45
author: 
created: 
title: "Align"
tags: []
signature: "pub struct Align"
---

A button showing the current alignment, and the drawn picker behind it.

# Examples

```rust
use crawrad_actions::inspector_model::editors::align::Align;
use crawrad_actions::inspector_model::editor::PropEditor;

let align = Align::new("button1".to_string(), "alClient");
assert_eq!(align.template(), "RowPick");
```
