---
id: link-pick-linkpick-choose
kind: doc
target_file: src/link_pick.rs
line_hint: 120
author: 
created: 
title: "LinkPick::choose"
tags: []
signature: "pub fn choose (& mut self , i : usize) -> Option < (usize , String) >"
---

The `i`-th item is chosen: the caret and its link, once; the list
closes.

# Examples

```rust
use crawrad_host::link_pick::{Linkable, LinkPick};
use std::path::Path;

let mut pick = LinkPick::default();
pick.open(5, Path::new("src"));
let (ticket, _) = pick.ask("foo").unwrap();
pick.answered(ticket, vec![Linkable::from(("foo".into(), None, "src/a.rs".into(), 0))]);
assert_eq!(pick.choose(0), Some((5, "[`foo`]".into())));
```
