---
id: inspector-model-proprow
kind: doc
target_file: src/inspector_model.rs
line_hint: 254
author: 
created: 
title: "PropRow"
tags: []
signature: "pub struct PropRow"
---

A property on screen: the name, the key and the editor.

# Examples

```rust
use crawrad_actions::inspector_model::editors::mute::Mute;
use crawrad_actions::inspector_model::PropRow;

let row = PropRow {
    key: "caption",
    label: "Caption".to_string(),
    depth: 0,
    editor: Mute::differing(),
    striped: false,
};
assert_eq!(row.key, "caption");
```
