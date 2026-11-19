---
id: props-props-field
kind: doc
target_file: src/props.rs
line_hint: 2222
author: 
created: 
title: "Props::field"
tags: []
signature: "pub fn field (default : & str) -> PropDesc"
---

The field of the data source a column shows.

# Examples

```rust
use crawrad_registry::Props;

assert_eq!(Props::field("").key, "field");
```
