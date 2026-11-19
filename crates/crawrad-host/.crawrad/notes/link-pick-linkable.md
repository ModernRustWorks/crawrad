---
id: link-pick-linkable
kind: doc
target_file: src/link_pick.rs
line_hint: 14
author: 
created: 
title: "Linkable"
tags: []
signature: "pub struct Linkable"
---

An item that can be linked: its name, the type it belongs to, and where.

# Examples

```rust
use crawrad_host::link_pick::Linkable;
use std::path::PathBuf;

let item = Linkable {
    name: "run".into(),
    container: Some("App".into()),
    file: PathBuf::from("src/main.rs"),
    line: 10,
};
assert_eq!(item.path(), "App::run");
```
