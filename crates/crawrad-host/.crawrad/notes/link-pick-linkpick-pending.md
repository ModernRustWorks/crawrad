---
id: link-pick-linkpick-pending
kind: doc
target_file: src/link_pick.rs
line_hint: 99
author: 
created: 
title: "LinkPick::pending"
tags: []
signature: "pub fn pending (& self) -> Option < u64 >"
---

The ticket still waiting for its answer.

# Examples

```rust
use crawrad_host::link_pick::LinkPick;
use std::path::Path;

let mut pick = LinkPick::default();
pick.open(0, Path::new("src"));
let ticket = pick.ask("term").unwrap().0;
assert_eq!(pick.pending(), Some(ticket));
```
