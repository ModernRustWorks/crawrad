---
id: field
kind: doc
target_file: src/lib.rs
line_hint: 30
author: 
created: 
title: "Field"
tags: []
signature: "pub struct Field"
---

One field of a generated component struct.

# Examples

```rust
use crawrad_insight::Field;

let f = Field { name: "enabled".into(), ty: "bool".into(), doc: None };
assert_eq!(f.name, "enabled");
```
