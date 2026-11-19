---
id: inspector-model-editors-choice-choice
kind: doc
target_file: src/inspector_model/editors/choice.rs
line_hint: 16
author: 
created: 
title: "Choice"
tags: []
signature: "pub struct Choice"
---

A drop-down over a closed set.

# Examples

```rust
use crawrad_actions::inspector_model::editors::choice::Choice;
use crawrad_actions::inspector_model::editor::PropEditor;

let ch = Choice::new(&["A", "B"], "A");
assert_eq!(ch.template(), "RowChoice");
```
