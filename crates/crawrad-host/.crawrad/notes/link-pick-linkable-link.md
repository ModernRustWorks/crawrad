---
id: link-pick-linkable-link
kind: doc
target_file: src/link_pick.rs
line_hint: 44
author: 
created: 
title: "Linkable::link"
tags: []
signature: "pub fn link (& self) -> String"
---

The link rustdoc resolves: `` [`Type::item`] ``.

# Examples

```rust
use crawrad_host::link_pick::Linkable;
use std::path::PathBuf;

let item = Linkable {
    name: "run".into(),
    container: Some("App".into()),
    file: PathBuf::from("lib.rs"),
    line: 0,
};
assert_eq!(item.link(), "[`App::run`]");
```
