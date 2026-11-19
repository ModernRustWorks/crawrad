---
id: navigation-place
kind: doc
target_file: src/navigation.rs
line_hint: 16
author: 
created: 
title: "Place"
tags: []
signature: "pub struct Place"
---

A caret position: the file, the line and the byte in it (0-based).

# Examples

```rust
use crawrad_host::navigation::Place;
use std::path::PathBuf;

let place = Place {
    file: PathBuf::from("src/main.rs"),
    line: 10,
    byte: 4,
};
assert_eq!(place.line, 10);
```
