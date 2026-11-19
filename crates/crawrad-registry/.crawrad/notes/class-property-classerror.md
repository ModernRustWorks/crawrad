---
id: class-property-classerror
kind: doc
target_file: src/class_property.rs
line_hint: 65
author: 
created: 
title: "ClassError"
tags: []
signature: "pub enum ClassError"
---

Why the document was rejected when checked against the catalog.

# Examples

```rust
use crawrad_registry::class_property::ClassError;

let err = ClassError::UnknownClass {
    node: "btn1".into(),
    class: "CustomButton".into(),
};
assert!(err.to_string().contains("CustomButton"));
```
