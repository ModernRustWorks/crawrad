---
id: link-pick-linkable-label
kind: doc
target_file: src/link_pick.rs
line_hint: 49
author: 
created: 
title: "Linkable::label"
tags: []
signature: "pub fn label (& self) -> String"
---

The row: `Type::item  file.rs:4` (1-based line).

# Examples

```rust
use crawrad_host::link_pick::Linkable;
use std::path::PathBuf;

let item = Linkable {
    name: "run".into(),
    container: Some("App".into()),
    file: PathBuf::from("src/main.rs"),
    line: 3,
};
assert_eq!(item.label(), "App::run  main.rs:4");
```
