---
id: link-pick-linkpick-ask
kind: doc
target_file: src/link_pick.rs
line_hint: 86
author: 
created: 
title: "LinkPick::ask"
tags: []
signature: "pub fn ask (& mut self , typed : & str) -> Option < (u64 , String) >"
---

Submits a query to search, returning the ticket if non-empty.

`typed` changed: the ticket of the question to ask, none for a
blank name (the list empties).

# Examples

```rust
use crawrad_host::link_pick::LinkPick;
use std::path::Path;

let mut pick = LinkPick::default();
pick.open(5, Path::new("src"));
let asked = pick.ask("widget");
assert!(asked.is_some());
```
