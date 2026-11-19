---
id: targets-tab-places-literal
kind: doc
target_file: src/targets/tab_places.rs
line_hint: 41
author: 
created: 
title: "literal"
tags: []
signature: "pub fn literal (place : & [i64]) -> String"
---

`place` as the slice literal the generated code carries.

# Examples

```rust
use crawrad_codegen::targets::tab_places::literal;

assert_eq!(literal(&[0, 1]), "&[0, 1]");
```
