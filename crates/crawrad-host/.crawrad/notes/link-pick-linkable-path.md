---
id: link-pick-linkable-path
kind: doc
target_file: src/link_pick.rs
line_hint: 36
author: 
created: 
title: "Linkable::path"
tags: []
signature: "pub fn path (& self) -> String"
---

`Type::item`, or `item` for an item of a module.

# Examples

```rust
use crawrad_host::link_pick::Linkable;
use std::path::PathBuf;

let item = Linkable {
    name: "func".into(),
    container: None,
    file: PathBuf::from("lib.rs"),
    line: 0,
};
assert_eq!(item.path(), "func");
```
