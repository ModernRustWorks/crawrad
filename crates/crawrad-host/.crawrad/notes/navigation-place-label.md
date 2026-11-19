---
id: navigation-place-label
kind: doc
target_file: src/navigation.rs
line_hint: 136
author: 
created: 
title: "place_label"
tags: []
signature: "pub fn place_label (p : & Place) -> String"
---

A place as the list shows it: `name.rs:42  folder` (1-based line).

# Examples

```rust
use crawrad_host::navigation::{place_label, Place};
use std::path::PathBuf;

let place = Place {
    file: PathBuf::from("src/main.rs"),
    line: 9,
    byte: 0,
};
assert_eq!(place_label(&place), "main.rs:10  src");
```
