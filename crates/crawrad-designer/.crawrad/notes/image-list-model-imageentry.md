---
id: image-list-model-imageentry
kind: doc
target_file: src/image_list_model.rs
line_hint: 259
author: 
created: 
title: "ImageEntry"
tags: []
signature: "pub struct ImageEntry"
---

An icon entry in the list.

An icon in the list. Lives in the document as `Value::Map { name, symbol }` —
the IR already knew how to express collections, and a property that only this editor understands
would not survive code generation.

# Examples

```rust
use crawrad_designer::image_list_model::ImageEntry;

let entry = ImageEntry {
    name: "Save".to_string(),
    symbol: "💾".to_string(),
};
assert_eq!(entry.name, "Save");
```
