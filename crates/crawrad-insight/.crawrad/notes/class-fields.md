---
id: class-fields
kind: doc
target_file: src/lib.rs
line_hint: 129
author: 
created: 
title: "class_fields"
tags: []
signature: "pub fn class_fields (designer_source : Option < & str > , class : & str ,) -> Option < (Vec < Field > , FieldSource) >"
---

Resolves component struct fields from designer source or generator.

The fields of `class`: from the project's `.designer.rs` when it has the
struct, else from the Makepad generator; `None` when neither knows it.

# Examples

```rust
use crawrad_insight::class_fields;

assert!(class_fields(None, "Button").is_some());
```
