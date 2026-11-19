---
id: inspector-model-namesplitter-layout
kind: doc
target_file: src/inspector_model.rs
line_hint: 649
author: 
created: 
title: "NameSplitter::layout"
tags: []
signature: "pub fn layout (& mut self , list_width : f64) -> (f64 , f64)"
---

Computes the name width and splitter position for a given list width.

The name width to draw in a list `list_width` wide, and where the strip
goes from the list's left edge. Before the first layout there is no
width, and the request is drawn as it is.

# Examples

```rust
use crawrad_actions::inspector_model::NameSplitter;

let mut splitter = NameSplitter::default();
let (name_w, strip_pos) = splitter.layout(300.0);
assert_eq!(name_w, 136.0);
```
