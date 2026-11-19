---
id: link-pick-linkpick-open
kind: doc
target_file: src/link_pick.rs
line_hint: 67
author: 
created: 
title: "LinkPick::open"
tags: []
signature: "pub fn open (& mut self , caret : usize , root : & Path)"
---

Opens the picker for a link at byte `caret` under `root`.

The list opens for a link at byte `caret` of the note, to the items
under `root`.

# Examples

```rust
use crawrad_host::link_pick::LinkPick;
use std::path::Path;

let mut pick = LinkPick::default();
pick.open(12, Path::new("src"));
assert!(pick.is_open());
```
