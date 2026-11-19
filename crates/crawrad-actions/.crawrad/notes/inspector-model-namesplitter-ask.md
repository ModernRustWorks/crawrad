---
id: inspector-model-namesplitter-ask
kind: doc
target_file: src/inspector_model.rs
line_hint: 660
author: 
created: 
title: "NameSplitter::ask"
tags: []
signature: "pub fn ask (& mut self , width : i32)"
---

What `inspector.set_name_width` asked, kept as asked.

# Examples

```rust
use crawrad_actions::inspector_model::NameSplitter;

let mut splitter = NameSplitter::default();
splitter.ask(150);
assert_eq!(splitter.requested, 150.0);
```
