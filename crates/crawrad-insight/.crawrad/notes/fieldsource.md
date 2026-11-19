---
id: fieldsource
kind: doc
target_file: src/lib.rs
line_hint: 50
author: 
created: 
title: "FieldSource"
tags: []
signature: "pub enum FieldSource"
---

Source from which component struct fields were retrieved.

Where the struct of a class was read from — so a caller (and a test) can
tell the project's own code from the generator's fallback.

# Examples

```rust
use crawrad_insight::FieldSource;

assert_eq!(FieldSource::Designer, FieldSource::Designer);
```
