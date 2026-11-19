---
id: manifest-propentry
kind: doc
target_file: src/manifest.rs
line_hint: 701
author: 
created: 
title: "PropEntry"
tags: []
signature: "pub struct PropEntry"
---

A property, in the file format.

# Examples

```rust
use crawrad_package::manifest::{Group, Kind, PropEntry};

let prop = PropEntry {
    key: "text".into(),
    display: "Text".into(),
    category: Group::Appearance,
    kind: Kind::Text,
    default: serde_json::json!(""),
    readonly: false,
    bindable: false,
    options: Vec::new(),
    ref_class: None,
    field_of: None,
    tree_collection: false,
    item_class: None,
    min: None,
    max: None,
    step: None,
    multiline: false,
    hidden_when: None,
};
assert_eq!(prop.key, "text");
```
