---
id: inspector-model-editors-lines-lines
kind: doc
target_file: src/inspector_model/editors/lines.rs
line_hint: 46
author: 
created: 
title: "Lines"
tags: []
signature: "pub struct Lines"
---

A button that opens the text pad, and the count beside it.

# Examples

```rust
use crawrad_actions::inspector_model::editors::lines::Lines;
use crawrad_actions::inspector_model::editor::PropEditor;

let l = Lines::new("memo1".to_string(), "lines", "hello\nworld");
assert_eq!(l.template(), "RowOpen");
```
