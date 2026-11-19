---
id: inspector-model-editors-mute-mute
kind: doc
target_file: src/inspector_model/editors/mute.rs
line_hint: 48
author: 
created: 
title: "Mute"
tags: []
signature: "pub struct Mute"
---

A row that shows and does not edit.

# Examples

```rust
use crawrad_actions::inspector_model::editors::mute::{Mute, READ_ONLY};
use crawrad_actions::inspector_model::editor::PropEditor;

let m = Mute::new("text", READ_ONLY);
assert_eq!(m.template(), "RowMute");
```
