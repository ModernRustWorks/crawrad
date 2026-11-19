---
id: external-externalprop
kind: doc
target_file: src/external.rs
line_hint: 205
author: 
created: 
title: "ExternalProp"
tags: []
signature: "pub struct ExternalProp"
---

One property a third party declares.

# Examples

```rust
use crawrad_ir::Value;
use crawrad_registry::Category;
use crawrad_registry::external::{ExternalKind, ExternalProp};

let prop = ExternalProp {
    key: "text".into(),
    display: "Text".into(),
    default: Value::Str(String::new()),
    category: Category::Identity,
    kind: ExternalKind::Text,
    readonly: false,
    bindable: false,
    options: Vec::new(),
    ref_class: None,
    tree_collection: false,
    item_class: None,
    multiline: false,
    hidden_when: None,
};
assert_eq!(prop.key, "text");
```
